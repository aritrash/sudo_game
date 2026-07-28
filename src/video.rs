use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use crossbeam_channel::{bounded, Receiver};
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;

pub struct VideoPlugin;

impl Plugin for VideoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_video_texture);
    }
}

// Component to hold the channel, texture handle, and keep the pipeline alive
#[derive(Component)]
pub struct VideoStream {
    pub receiver: Receiver<Vec<u8>>,
    pub target_image: Handle<Image>,
    pub pipeline: gst::Pipeline,
}

// Ensure GStreamer pipeline stops playing and frees memory when the entity drops
impl Drop for VideoStream {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }
}

/// Spawns the GStreamer pipeline and returns the handles needed by Bevy
pub fn spawn_video_pipeline(
    images: &mut ResMut<Assets<Image>>,
    video_path: &str,
    width: u32,
    height: u32,
) -> (Handle<Image>, Receiver<Vec<u8>>, gst::Pipeline) {
    
    // 1. Create a blank Bevy texture that we will overwrite every frame
    let image = Image::new_fill(
        Extent3d { width, height, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 255], // Start black
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    let image_handle = images.add(image);

    // 2. Create the cross-thread channel (buffer 2 frames)
    let (sender, receiver) = bounded::<Vec<u8>>(2);

    // 3. Resolve absolute path for Windows safely
    let path = std::env::current_dir().unwrap().join(video_path);
    let uri = gst::glib::filename_to_uri(&path, None)
        .expect("Failed to convert path to GStreamer URI")
        .to_string();

    // 4. Build the exact GStreamer decoding string
    let pipeline_str = format!(
        "uridecodebin uri={} ! videoconvert ! video/x-raw,format=RGBA,width={},height={} ! appsink name=sink drop=true max-buffers=2",
        uri, width, height
    );
    
    let pipeline = gst::parse::launch(&pipeline_str)
        .unwrap()
        .downcast::<gst::Pipeline>()
        .unwrap();
    
    // 5. Connect to the appsink to pull the raw bytes
    let sink = pipeline
        .by_name("sink")
        .unwrap()
        .downcast::<gst_app::AppSink>()
        .unwrap();
    
    sink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            .new_sample(move |appsink| {
                let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or_else(|| {
                    gst::element_error!(appsink, gst::ResourceError::Failed, ("Failed to get buffer"));
                    gst::FlowError::Error
                })?;
                
                let map = buffer.map_readable().map_err(|_| {
                    gst::element_error!(appsink, gst::ResourceError::Failed, ("Failed to map buffer"));
                    gst::FlowError::Error
                })?;
                
                // Fire the bytes across the channel to the Bevy thread
                let _ = sender.send(map.as_slice().to_vec());
                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );

    // 6. Start playing the video loop
    pipeline.set_state(gst::State::Playing).unwrap();

    (image_handle, receiver, pipeline)
}

/// Bevy System: Pulls bytes from the channel and pushes them to the GPU texture
fn update_video_texture(
    mut images: ResMut<Assets<Image>>,
    query: Query<&VideoStream>,
    mut frame_count: Local<u32>, 
) {
    for stream in query.iter() {
        // 1. POLL GSTREAMER FOR ERRORS AND END-OF-STREAM
        if let Some(bus) = stream.pipeline.bus() {
            for msg in bus.iter() {
                use gst::MessageView;
                match msg.view() {
                    MessageView::Eos(_) => {
                        // THE FIX: Instantly rewind the video to 00:00:00 when it ends
                        let _ = stream.pipeline.seek_simple(
                            gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                            gst::ClockTime::ZERO,
                        );
                    }
                    MessageView::Error(err) => {
                        eprintln!(
                            "\n[GST ERROR] {} \n[GST DEBUG] {:?}\n",
                            err.error(),
                            err.debug()
                        );
                    }
                    MessageView::Warning(warn) => {
                        eprintln!("[GST WARNING] {}", warn.error());
                    }
                    _ => {}
                }
            }
        }

        // 2. PULL THE LATEST FRAME
        if let Some(latest_frame) = stream.receiver.try_iter().last() {
            if *frame_count == 0 {
                println!("\n>>> FIRST FRAME RECEIVED! PIPELINE IS ALIVE! <<<\n");
            }
            *frame_count += 1;

            if let Some(image) = images.get_mut(&stream.target_image) {
                image.data = latest_frame;
            }
        }
    }
}
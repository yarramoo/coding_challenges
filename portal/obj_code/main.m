#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

// Custom delegate to handle video frame output
@interface VideoDelegate : NSObject<AVCaptureVideoDataOutputSampleBufferDelegate>
@end

@implementation VideoDelegate

- (void)captureOutput:(AVCaptureOutput *)output
    didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer
    fromConnection:(AVCaptureConnection *)connection
{
    // Extract the image buffer from the sample buffer
    CVImageBufferRef imageBuffer = CMSampleBufferGetImageBuffer(sampleBuffer);
    if (!imageBuffer) {
        NSLog(@"No image buffer available.");
        return;
    }

    // Lock the pixel buffer to access its data
    CVPixelBufferLockBaseAddress(imageBuffer, kCVPixelBufferLock_ReadOnly);

    // Get frame dimensions
    size_t width = CVPixelBufferGetWidth(imageBuffer);
    size_t height = CVPixelBufferGetHeight(imageBuffer);

    // Unlock the pixel buffer after reading
    CVPixelBufferUnlockBaseAddress(imageBuffer, kCVPixelBufferLock_ReadOnly);

    // Print the frame dimensions
    NSLog(@"Captured frame with dimensions: %zux%zu", width, height);
}

@end

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        NSLog(@"Starting video capture...");

        // Create the AVCaptureSession
        AVCaptureSession *session = [[AVCaptureSession alloc] init];
        session.sessionPreset = AVCaptureSessionPresetHigh;

        // Get the default video device (FaceTime camera)
        AVCaptureDevice *device = [AVCaptureDevice defaultDeviceWithMediaType:AVMediaTypeVideo];
        if (!device) {
            NSLog(@"Failed to access the camera.");
            return -1;
        }

        // Create a device input and add it to the session
        NSError *error = nil;
        AVCaptureDeviceInput *input = [AVCaptureDeviceInput deviceInputWithDevice:device error:&error];
        if (!input) {
            NSLog(@"Failed to create camera input: %@", error.localizedDescription);
            return -1;
        }
        [session addInput:input];

        // Create a video data output and set its delegate
        AVCaptureVideoDataOutput *output = [[AVCaptureVideoDataOutput alloc] init];
        VideoDelegate *delegate = [[VideoDelegate alloc] init];
        [output setSampleBufferDelegate:delegate queue:dispatch_get_main_queue()];
        [session addOutput:output];

        // Start the session
        [session startRunning];
        NSLog(@"Video capture session started.");

        // Run the run loop to keep the program alive
        [[NSRunLoop currentRunLoop] run];
    }
    return 0;
}

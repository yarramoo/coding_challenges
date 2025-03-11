#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

// Rust callback to process frames
void process_frame(const void* baseAddress, int width, int height);

// VideoDelegate implementation
@interface VideoDelegate : NSObject<AVCaptureVideoDataOutputSampleBufferDelegate>
@end

@implementation VideoDelegate

- (void)captureOutput:(AVCaptureOutput *)output
    didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer
    fromConnection:(AVCaptureConnection *)connection
{
    // Get the image buffer
    CVImageBufferRef imageBuffer = CMSampleBufferGetImageBuffer(sampleBuffer);
    if (!imageBuffer) {
        NSLog(@"No image buffer available.");
        return;
    }

    // Lock the image buffer
    CVPixelBufferLockBaseAddress(imageBuffer, kCVPixelBufferLock_ReadOnly);

    // Get frame properties
    int width = (int)CVPixelBufferGetWidth(imageBuffer);
    int height = (int)CVPixelBufferGetHeight(imageBuffer);
    const void* baseAddress = CVPixelBufferGetBaseAddress(imageBuffer);

    // Call Rust function to process the frame
    process_frame(baseAddress, width, height);

    // Unlock the image buffer
    CVPixelBufferUnlockBaseAddress(imageBuffer, kCVPixelBufferLock_ReadOnly);
}

@end

static VideoDelegate *delegateInstance;

void* setup_video_capture() {
    @try {
        AVCaptureSession *session = [[AVCaptureSession alloc] init];
        AVCaptureDevice *device = [AVCaptureDevice defaultDeviceWithMediaType:AVMediaTypeVideo];
        if (!device) {
            NSLog(@"Failed to access the camera.");
            return NULL;
        }

        AVCaptureDeviceInput *input = [AVCaptureDeviceInput deviceInputWithDevice:device error:nil];
        if (!input) {
            NSLog(@"Failed to create input for camera.");
            return NULL;
        }

        [session addInput:input];

        AVCaptureVideoDataOutput *output = [[AVCaptureVideoDataOutput alloc] init];
        delegateInstance = [[VideoDelegate alloc] init];  // Retain delegate
        [output setSampleBufferDelegate:delegateInstance queue:dispatch_get_main_queue()];
        [session addOutput:output];

        [session startRunning];
        NSLog(@"Video capture session started.");
        return (__bridge void*)session;
    }
    @catch (NSException *exception) {
        NSLog(@"Exception occurred: %@", exception);
        return NULL;
    }
}

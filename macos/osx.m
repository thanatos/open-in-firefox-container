#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>


void osx_handle_url(const char *c_url);


@interface AppDelegate : NSObject<NSApplicationDelegate>
- (void)applicationWillFinishLaunching:(NSNotification *)aNotification;
- (void)handleURLEvent:(NSAppleEventDescriptor*)event
        withReplyEvent:(NSAppleEventDescriptor*)replyEvent;
@end


@implementation AppDelegate
// Most of the implementation here is from this excellent Stack Overflow
// question, and other sources about the Internet:
// https://stackoverflow.com/questions/1991072/how-to-handle-with-a-default-url-scheme
// In particular, Thomas Zoechling et al.'s answer was quite helpful.
//
// https://stackoverflow.com/questions/23826449/shell-script-as-default-browser
// was also quite helpful, and some of the below also comes from that.
// (Though that answer has this code in applicationDidFinishLaunching, which
// appears to be too late.)

- (void)applicationWillFinishLaunching:(NSNotification *)aNotification
{
    NSNotification *notification_sink __unused = aNotification;

    NSAppleEventManager *event_manager = [NSAppleEventManager sharedAppleEventManager];
    [event_manager
        setEventHandler:self
            andSelector:@selector(handleURLEvent:withReplyEvent:)
          forEventClass:kInternetEventClass
             andEventID:kAEGetURL];
    // This came from another Internet source. What does it do?
    [event_manager
        setEventHandler:self
            andSelector:@selector(handleURLEvent:withReplyEvent:)
          forEventClass:'WWW!'
             andEventID:'OURL'];

    CFStringRef bundleID = (__bridge CFStringRef)[[NSBundle mainBundle] bundleIdentifier];
    LSSetDefaultHandlerForURLScheme(CFSTR("http"), bundleID);
    LSSetDefaultHandlerForURLScheme(CFSTR("https"), bundleID);
}

- (void)handleURLEvent:(NSAppleEventDescriptor*)event
        withReplyEvent:(NSAppleEventDescriptor*)replyEvent
{
    NSAppleEventDescriptor *reply_sink __unused = replyEvent;

    NSString* url = [[event paramDescriptorForKeyword:keyDirectObject]
                        stringValue];
    const char *c_url = [url cStringUsingEncoding:NSUTF8StringEncoding];
    osx_handle_url(c_url);
    NSApplication *app = [NSApplication sharedApplication];
    [app terminate:app];
}

@end


int osx_objc_main(int argc, const char *argv[]) {
    NSApplication *app = [NSApplication sharedApplication];
    AppDelegate *delegate = [AppDelegate alloc];
    [app setDelegate: delegate];
    return NSApplicationMain(argc, argv);
}

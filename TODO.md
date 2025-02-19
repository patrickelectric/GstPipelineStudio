## 0.1.0

- [x] Fix c.fill issue
- [x] Create Element structure with pads and connections
- [x] Get a list of GStreamer elements in dialog add plugin
- [x] Add plugin details in the element dialog
- [x] Draw element with its pad
- [x] Be able to move the element on Screen
- [x] Create connection between element
- [x] create contextual menu on pad or element
- [x] save/load pipeline
- [x] Run a pipeline with GStreamer
- [x] Run the pipeline with GStreamer
- [x] Control the pipeline with GStreamer
- [x] select nodes/links with a Trait Selectable
- [x] be able to remove a link by selecting it
- [x] Connect the logs to the window
- [x] Define the license
- [x] crash with x11 on contextual menu
- [x] open multiple times dialog (About) prevent to close it.
- [x] remove useless code from graphview
- [x] Move render to a specific module
- [x] Move GST render to a specific module

## 0.2.0

### Graphview

- [x] Remove a port from a node when its possible (Presence support)
- [x] Implement graphview unit test
- [x] Add a css class for pad (presence always or sometimes)
- [x] Add properties to Port to store some specific value (ie Caps)
- [x] Unable to connect a port which is already connected
- [x] Unable to connect port with same directions (in/in, out/out)

### GStreamer:

- [x] Add seek support
- [x] Use of gtk4paintablesink

### app

- [x] Check that a node accepts to create a port on request (input/output)
- [x] Render the parse launch line in a message box
- [x] Prevent to create a pad in an element without the template
- [x] Check the pipeline validity
- [x] Save node position in XML
- [x] Auto-save the graph
- [x] Logger in file/app all over the app
- [x] Property window in the main window
- [x] Connect the GPS status to GST status
- [x] Display position and duration
- [x] Seek to position with slider
- [x] One listbox with elements and one listbox with favorites in the app dashboard
- [x] See the link creation with a dashed line
- [x] Display pad properties with tooltip hover
- [x] Add preferences dialog
- [x] Create a window for the video output

### infra

- [x] Icon install
- [x] Flatpak infrastructure

## 0.2.1

### app

- [x] Can set pad properties to be used during the pipeline generation. See videomixer_alpha.xml
- [x] Support gtk4paintablesink with playbin
- [x] Display a pipeline properties dialog (list elements)

## 0.2.2

### app

- [x] Remove quit as it's unnecessary with close button
- [x] Remove the close button in dialogs (properties etc.)
- [x] Unable to use flags in playbin3
- [x] the desktop icon execs gps_pipeline_studio
- [x] move burger menu on the right

### Graphview

- [x] Update node description on property removal

## TODO

### Graphview

- [ ] create a crate for graphview/node/port

### GStreamer:

- [ ] Implement pipeline unit test

### app

- [ ] Control the connection between element
  - [ ] unable to connect element with incompatible caps.
- [ ] Add multiple graphviews with tabs.
- [ ] Implement graph dot render/load
- [ ] Implement a command line parser to graph
- [ ] handle the caps setter element
- [ ] Add probes on each pad to monitor the pipeline
- [ ] Render a media file
- [ ] Offer compatible element to a pad (autorender)
- [ ] Display tags/meta/message detected
- [ ] Change TreeView to ListView
- [ ] Implement zoom on the view (https://gitlab.gnome.org/World/obfuscate/-/blob/master/src/widgets/drawing_area.rs)
- [ ] Settings: add a log level selection
- [ ] reopen the last log on prematured exit (crash)
- [ ] Play/pause should be prevented until the pipeline is ready
- [ ] Filter the elements by class/rank etc.

### CI/Infra

- [ ] Create a macos/windows job

## bugs

- [ ] check that element exists before creating it on file load.
- [ ] Combo box is not well selected if the value is not linear such as flags. See flags in playbin

# NERF

Nerf is (yet another) rust GUI lib. It is heavily inspired by Flutter, and is designed to build apps that could run on any plateforms, such as windows / linux / macOS, but also web, android, etc.

It was created after seing how huge and complex other GUI libs were. The simple counter example with Iced is 8MB, and the whole repo is 60k lines of code. When I tried it out, the compile time was huge. The counter Nerf example is 5K lines of code (when I write this readme, this will increase in the future) and the executable is 2MB.

I wanted something simple, and straightforward. A lib that anyone could dive into, and understand how it works.

## Features

For now, Nerf is in early development. Therefore, only a few types of widgets are currently supported.

- Widget tree system
- Basic widget rendering 
- Text rendering

## Dependencies

Nerf has a few dependencies, but most of them are optionnal and can be included with features.

- winit: window creation and management, essential.
- softbuffer: provides a 2D pixel buffer to draw on from the winit handle.
- skia: draw engine. It is enabled by default, but can be disabled with the --no-default-features flag. When disabled, the most basic rendering operations have fallbacks, but they are much slower. It is recommanded to use skia, unless executable size should as small as possible.
- cosmic-text: text rendering. This is disabled bu default, and any application that uses text rendering should add it. It does considerably increase the executable size.


## How to use

### Installation

Nerf is not yet published on crates.io, so you have to clone the repo and reference it from your project. This can easily be done in the Toml file:

```toml
[dependencies]
nerf = { path = "path/to/nerf" }
```

### Basic usage

At it's core, nerf is not much more than a widget tree. The one important trait is the `Widget` trait, which defines a widget behaviour. The `Widget` trait has three methods:

draw:
```rust
fn draw(&self, canvas: &mut Canvas, rect: softbuffer::Rect);
```
The draw function is called whenever the application requests a redraw. There are usually to implementations:
- some widgets will actually draw something on the canvas
- others will recursively call the draw function on their children

min_space_requirements:
```rust
fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement);
```
This allows to have information on how widgets wished to be layout. This will allow the app to give them the required space, if available. It is important to note that this can not always be respected, and widgets should be able to handle smaller sizes. We can't stop a app_user to making a window smaller than the minimum size of a widget.

handle_event:
```rust
fn handle_event(&mut self, event: InputEvent, rect: softbuffer::Rect) -> EventResponse;
```
The handle event function is called whenever an event is received. It should be recursively called on all children.
When a widget uses that event, they must notifiy the parent by returning a event response flags, such as the request redraw for example.

With Nerf, the idea is that widets implement their logic and own their data. Any desired behaviour is made by creating a widget, and adding it to the widget tree. The widget tree is then passed to the application, which will handle the rendering and events. This follows the rust philosophy of ownership, and non-representable states. For example, a connection page widget does not have data for a user : therefore, any null data is not representable. The connected page however does have user data, and can use it.

### Example

Let's implement the classic counter example (the full example is in examples/counter.rs).

First, let's create a widget that will contain our data.

```rust
struct Counter {
    // state of the widget
    count: u32,
}
```

While we could implement manually the drawing, event handling of our counter, Nerf provides basic widgets that can be used. Our counter will therfore have two childs, a text widget to display the count, and a button to increment the count.

```rust
struct Counter {
    // state of the widget
    count: u32,
    // the button that will throw callbacks
    button: Box<dyn Widget>,
    // the display text. We keep it here, to be able to reference and mofify it.
    text: Box<Text>,
}
```

Here, the button is a dyn widget, because we don't care a lot about it. We kept the text strong type to change it's value later.

Now, let's implement the widget trait for our counter.

```rust
impl Widget for Counter {
    [...]
}
```

Let's start with the draw function. All we need to draw is the text. However, we'll add a background color to make it more visible.
As we keep a reference to the text widget, we can't have it be a child of our background: widgets have a unique owner. We will draw our background behind the button, which is a widget that does not get drawn, and only have a behaviour. Therefore, we will draw the button and the text. 

```rust
fn draw(&self, canvas: &mut Canvas, rect: softbuffer::Rect) {
    self.button.draw(canvas, rect);
    self.text.draw(canvas, rect);
}
```

See how simple this is ? The app will take care of the layout, and will provide us with the rect to draw in. More detail on this with the next function:

```rust
fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
    (
        WidgetSizeRequirement::Fixed(unsafe {NonZeroU32::new_unchecked(200)}),
        WidgetSizeRequirement::Fixed(unsafe {NonZeroU32::new_unchecked(70)}),
    )
}
```

Let's specify what space requirements we want. We could use the space requirements of our children, but here let's just say we want a fixed size. when drawn, the app will do it's best to provide us with the requested size.

Finally, let's handle the events. We want to handle the button click, and increment the counter.

```rust
fn handle_event(&mut self, event: InputEvent, rect: softbuffer::Rect) -> EventResponse {

    let result = self.button.handle_event(event, rect);
    if result.contains(EventResponse::CALLBACK) {
        // in that case, increment the counnter and update the text.
        self.count += 1;
        self.text.set_text(self.count.to_string())
    }

    let result = result | EventResponse::REDRAW_REQUEST | !EventResponse::CALLBACK;

    result
}
```

If the button is pressed, it will return a callback flag. If we see this flag, we increment the counter and update the text. It is then important to return a draw request repsonse ourselves, to tell the app that we need to be redrawn. Here, we could simply return request redraw, but in more complex architectures, we might not now what events are thrown through our widgets. Also, it is worth noticing we removed the callback flag. This is left to the user, but here as we consumed the event, I found it better to keep it to this widget.

Finally, let's implement a constructor for our counter.

```rust
pub fn new() -> Box<Counter> {
    Box::new(
        Counter {
            count: 0,
            button: Background::new(
                Color::rgb(200, 255, 200),
                Button::new(Empty::expand()),
            ),
            text: Text::new(
                "0".to_string(),
                TextStyle::default()
                    .sized(30.0)
                    .styled(FontStyle::Italic)
            ),
        }
    )
}
```

Here, simply create a new counter with an initial state, assign the button and a text. You will notice the button is behind a background. This is why I kept it under a dyn widget: it is not important to know what it is, as we won't interact with it. The events will be redirected from the background to the button, and the button will throw a callback when pressed that we will also receive through the background. You could think of this as a "background button". But in Nerf, widgets won't assume any behaviour, and will only be used for their sole purpose. therfore, a  button will only be used to throw callbacks, and a background will only be used to draw a background. If we want to have a background button, we will create a button, and add a background to it, as demonstrated here.

To avoid infinite size structs, most widgets are held in boxes in Nerf. It is however possible not to: our counter could have a straight text widget. But here, the app will expect a struct for the root, so our constructor returns a box.

Finally, let's create our app.

```rust
fn main() {
    let app = App::new(
        Align::new(
            Alignment::CENTER,
            Center::new(Counter::new()),
        )
    );

    // Run the app.
    app.run()
}
```

Here, simply create an app, put the counter as the root, and start it. You can see I added a center for convenience.

### Going further

As shown, my main idea behind Nerf is that widgets own their data. As this is in early development, this is still experimental and might be too limiting to be practical. However, I think this is a good way to go, and I will try to make it work.

If you want to try it out and encounter any issues, or have any questions / remarks, please let me know !
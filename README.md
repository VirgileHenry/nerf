# NERF

Nerf is (yet another) rust GUI lib. It is heavily inspired by Flutter, and is designed to build apps that could run on any plateforms, such as windows / linux / macOS, but also web, android...

## Why another GUI lib ?

Nerf started as a hobby side project. It is made to be simple to use, while allowing user to write anything in rust, giving access to it's growing ecosystem.

Furthermore, there are some design choices that were important to me:

- Explicit: everything is drawn exactly as you described it. No hidden magic, no implicit or expected behavior. Each widget will do it's sole and only purpose. A good example is that a `SizedBox` will only size it's child, and nothing else. It will not draw any background, won't try to align or anything. If you want to display anything on the screen, you will need to use widgets that are made for that purpose.

- Pixel coordinate system: The entire library is based on pixel coordinates, represented bu `u32`. This allows for easy and precise positioning of widgets, and makes it easy to reason about the layout of your app. This have a drawback: trying to center an even-sized widget in a odd-sized container will result in a 1px offset.

## Status

This gui lib is still in very early developement, and is not ready for any kind of production use.
Related to [[Dynamic hitboxes and hurtboxes]]
Basically, this is what the bevy discord had to say:


By "Griffin":
> Here's a 2d SDF of an n-ellipse [https://www.shadertoy.com/view/Wd33Rr](https://www.shadertoy.com/view/Wd33Rr "https://www.shadertoy.com/view/Wd33Rr")
> The simplest example right now of the kind of shader you'd want for this would be this: [https://github.com/bevyengine/bevy/blob/main/examples/shader/post_processing.rs](https://github.com/bevyengine/bevy/blob/main/examples/shader/post_processing.rs "https://github.com/bevyengine/bevy/blob/main/examples/shader/post_processing.rs")
> Since you would want it as an overlay rather than a material
> Other examples of this kind of shader [https://github.com/eliotbo/bevy_shadertoy_wgsl](https://github.com/eliotbo/bevy_shadertoy_wgsl "https://github.com/eliotbo/bevy_shadertoy_wgsl") [https://github.com/torsteingrindvik/bevy-vfx-bag](https://github.com/torsteingrindvik/bevy-vfx-bag "https://github.com/torsteingrindvik/bevy-vfx-bag") [https://github.com/bevyengine/bevy/tree/main/crates/bevy_core_pipeline/src/fxaa](https://github.com/bevyengine/bevy/tree/main/crates/bevy_core_pipeline/src/fxaa "https://github.com/bevyengine/bevy/tree/main/crates/bevy_core_pipeline/src/fxaa") [https://github.com/bevyengine/bevy/tree/main/crates/bevy_core_pipeline/src/bloom](https://github.com/bevyengine/bevy/tree/main/crates/bevy_core_pipeline/src/bloom "https://github.com/bevyengine/bevy/tree/main/crates/bevy_core_pipeline/src/bloom")
> About SDFs in general [https://iquilezles.org/articles/distfunctions/](https://iquilezles.org/articles/distfunctions/ "https://iquilezles.org/articles/distfunctions/")
> [Inigo Quilez](https://iquilezles.org/articles/distfunctions/)
   Articles on computer graphics, math and art
   https://iquilezles.org/logo.jpg
   ![Inigo Quilez](https://images-ext-2.discordapp.net/external/MOWrFWVdLK1TEq64kxKpSxMLRoWvncu0mrXOvsIm3aQ/https/iquilezles.org/logo.jpg?width=400&height=225)
   They also have a youtube channel: [https://www.youtube.com/@InigoQuilez](https://www.youtube.com/@InigoQuilez "https://www.youtube.com/@InigoQuilez")
   [Inigo Quilez](https://www.youtube.com/@InigoQuilez)
   "Painting with Maths" is all about using mathematics with purely artistic goals within the medium of computer graphics. We'll be designing expressions and deriving formulas for things like dolphins, elephants and landscapes. If you like math, computer graphics, shaders, rendering or fractals, but want to peek into the world of art, this channel ...
   https://yt3.ggpht.com/ytc/AMLnZu_KeCpgZNpuxL46IQTwZ8_MfUMTg04osS2GnDbA=s900-c-k-c0x00ffffff-no-rj 
   ![Image](https://images-ext-2.discordapp.net/external/NQ6g4D1dewRzZCa2S76JIvd3-U9U2HBLK8xyBGj6AeY/https/yt3.ggpht.com/ytc/AMLnZu_KeCpgZNpuxL46IQTwZ8_MfUMTg04osS2GnDbA%3Ds900-c-k-c0x00ffffff-no-rj?width=80&height=80)
    There's also a crate for converting SDFs to a mesh: https://docs.rs/fast-surface-nets/latest/fast_surface_nets/ "https://docs.rs/fast-surface-nets/latest/fast_surface_nets/"
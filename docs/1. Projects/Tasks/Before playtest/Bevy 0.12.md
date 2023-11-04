[Migration guide](https://bevyengine.org/learn/migration-guides/0.11-0.12/)

[Blog post](https://bevyengine.org/news/bevy-0-12/)
Picks:
- https://bevyengine.org/news/bevy-0-12/#multiple-asset-sources allows loading assets through online sources. Could be big for net play.
- https://bevyengine.org/news/bevy-0-12/#embedded-assets Old solution for assets in binary had problems
	- What assets should one embed?
- https://bevyengine.org/news/bevy-0-12/#improved-hot-reloading-workflow Hot reload is now a cargo feature, just disable it when building for release
- https://bevyengine.org/news/bevy-0-12/#deferred-rendering + https://bevyengine.org/news/bevy-0-12/#material-extensions
	- Could allow some stylistic shaders
	- Could allow to ditch hanabi for a more custom particle system
		- Volumetric vertex shader particles
- https://bevyengine.org/news/bevy-0-12/#one-shot-systems - Potentially interesting, not sure what for
	- Events?
	- Game state transitions?
- https://bevyengine.org/news/bevy-0-12/#ui-materials - UI shaders to signal things in the shop
- https://bevyengine.org/news/bevy-0-12/#ui-node-outlines - Actual UI outlines
- https://bevyengine.org/news/bevy-0-12/#unified-time
	- Reconsider current "Once per frame" -solution in favor of `FixedUpdate` schedule
- https://bevyengine.org/news/bevy-0-12/#gamepadbuttoninput Have a look at input handling
- https://bevyengine.org/news/bevy-0-12/#sceneinstanceready-event can be used to have a proper loading screen
- https://bevyengine.org/news/bevy-0-12/#animationplayer-api-improvements
	- Will probably cause major cleanup
- https://bevyengine.org/news/bevy-0-12/#ignore-ambiguous-components-and-resources
	- Do another pass fixing ambiguity
	- Add err logging in case future ambiguity is introduced
- https://bevyengine.org/news/bevy-0-12/#reduced-tracing-overhead better tracking


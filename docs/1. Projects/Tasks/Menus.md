For stuff like pause and main menus

- Represent with a json object
	- root is the first level.
	- Each node has a heading and either subs(objects list) or action(string) as values for directional keys.

```json
{
	heading: 'Main menu',
	subs: [
		{
			heading: 'Options'
			subs: [...]
		},
		{
			heading: 'Play'
			subs: [...]
		},
		...
		{
			heading: 'Quit'
			action: 'quit'
		}
	]
}
```

- [ ] Load a temporary "Menu mode" to input parsers where the moves are stuff like "Up", "Down", "Accept" and "Back"
- [ ] Define a basic main menu
	- [ ] Container where cursor can move between selection options
	- [ ] Selecting an option does something
- [ ] Ignore that menu for dev purposes
	- [ ] Also maybe shorten pre-round
- [ ] Dunno about the menu structure given, it doesn't really scale

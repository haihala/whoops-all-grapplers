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

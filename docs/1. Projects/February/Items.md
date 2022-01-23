- [ ] Acquire currency
- [ ] Buy menu
	- [ ] Menu input parsing (fully event based)
- [ ] Basic items
	- [ ] Projectile active 6i (gun with limited bullets)
	- [ ] Basic damage buff passive
	- [ ] Parry
		- [ ] Open and close the parry window
		- [ ] Ignore damage, hitstun and knockback on a parry
		- [ ] Something (Gi?) of the old masters, similtaneously refering to Daigo and Ken.
- [ ] What if you had a pool of random things you could buy instead of always being able to buy everything
	- [ ] Maybe pin some items with mostly a random pool
- [ ] What you items had tiers and you gotta by an amount of things from a tier to unlock the next
	- [ ] Maybe max items bought per tier? (probably not)
	- [ ] Maybe unlocking a tier automatically gives you something (meter on tier 2, cancels on tier 3 and so on)

> Instead of an inventory component, have a wallet component and the. Each item is a component that is optionally queried if it changes behavior or has a system of its own.

- Items are components, usually empty
- Gun component tracks it's own ammo
	- How to check for ammo before shooting?

> Ft5, five tiers of items, you can guarantee one item per tier. It rolls two others for each tier. New tiers are unlocked based on how many rounds the leading plater has (0-2, 1-2 and 2-2 all mean that tiers 0, 1 and 2 are open)
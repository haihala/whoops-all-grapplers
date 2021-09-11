How pressing buttons leads to a move coming out.

# Pipeline
1. Common
	1. Controller connector
		1. Has a collection of controllers without characters
		2. Adds new controllers to the list 
		3. If a controller is without a player and vice versa, connects them
	2. Input collector
		1. Reads input for the controller
		2. Cares about a stick(or dpad), pressed buttons and released buttons
		3. Parses input into diffs, which are placed into the [[#Diff buffer]]
2. Character specific
	1. Parser
		1. Reads the [[#Diff buffer]] and updates the [[#Move buffer]]
		2. Handles prioritizing what moves come out
		3. Checks for conditions that influence what action to select
			1. Airborne
	2. Executor
		1. Reads the [[#Move buffer]]
		2. Checks for the rest of the relevant conditions
			1. Stunned
			2. Mid attack
		3. If appropriate, triggers the action
		4. Else advances the current action if any

# Diff buffer
Buffer of input changes. Frequently cut to optimize. Could be used to reconstruct the game if it was stored somewhere.

Stores a bunch of input frame objects, which have:
- What frame they were created
- If the stick moved, what is the new position
- If buttons were pressed, which ones
- If buttons were released, which ones

# Move buffer
Has two optional enum fields, only one of which is some at a time. One is for generic actions like [[movement]], the other is for character specific actions like attacks and special moves.

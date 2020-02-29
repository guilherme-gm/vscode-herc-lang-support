# Command Documentation Standard

This file aims to describe how the script commands are meant to be documented so we:
- have a format that is readable by the language server
- have everything filled in a meaningful way that could provide helpfull hints to the user
- allow us to programatically use those information.


## Format
Each comand must have a JSON file with the same name in the `script_commands` folder. For example, for `getitem` a file named `getitem.json` must exists.

The file content must be in the following format:
```
{
	"<command name>": {
		"params": {
			"<parameter name>": {
				"type": "<parameter type>",
				"default": "<parameter default value>",
				"doc": ["<documentation strings>"]
			}
		},
		"signatures": [
			[
				"<signature 1 parameter 1>"
			],
			[
				"<signature 2 parameter 1>",
				"<signature 2 parameter 2>",
			]
		],
		"doc": [
			"<documentation strings>"
		],
		"return": "<command return type>",
		"deprecated": <deprecated: true/false>
	}
}
```

- In `params` you must specify all parameters that the command has, even if it is only used in some of its signatures. The `type` value must be one of the types specified in the Types section below.
- In `signatures`, an array must be created for each of the command's possible signatures, and each parameter must refer to the parameter name written in `params` section
- For all `doc` sections, it is an array of strings that are later joined by a space. So if you DO WANT to break a line, you must write \n. This area accepts markdown.
- In `return` you must specify the return type (from the Types section below)
- Finally, for `deprecated`, `true` means that this command is deprecated and should not be used anymore.

## Types
Parameter and return types are divided in 2 main categories that are then subdivided in smaller groups.

The main categories are **Basic Types**, that is, types that are actually part of the language, and **Composite Types**, that are specifications of the Basic Types.

### Basic Types
Those are types that Hercules' engine actually understands or that are workarounds for the language server. They are currently 4:
- `number`: A numerical, integer, value.
- `string`: A text.
- `label`: Represents a label (like `OnInit`, `S_MySub`, etc)
- `unknown`: When we don't really know what it is. This was initially implemented to allow us to auto-generate the documentation as we don't have a way to set every type in it.

### Composite Types
Those are specifications of the basic types, they were created to allow us to do further checking and provide better, meaningfull, information to the developer. Internally, they are mapped to one or more basic type.

For example, one of the signatures for `getitem` has as its first parameter an Item ID, which is a number, so we say that its type is `item`, so **in the future we may be able to check when you are calling getitem without a valid Item and let you know!**

Those are the Composite Types:

#### `item`
**Basic Type:** `number`

Specifies an item ID, numerical or a constant.

Example:

`getitem(Apple, 1)`
- `Apple` is an `item`

----

#### `itemName`
**Basic Type:** `string`

Specifies a `string` that contains the name of an **item**.

Example:

`getitem("Apple", 1)`

(Even though this is not recommended)

----

#### `function`
**Basic Type:** `string`

Specifies a `string` that contains the name of a **function**.

Example:
`callfunc("myfunc", ...)`
- `"myfunc"` is a `function`

---

#### `map`
**Basic Type:** `string`

Specifies a `string` that contains the name of a **map**.

---

#### `bool`
**Basic Type:** `number`

Represents a boolean value, i.e. `true` (1) or `false` (0).

---

#### `gid` and `rid`
**Basic Type:** `number`

Specifies a `number` that represents an unit GID or RID.

---

#### `textLabel`
**Basic Type:** `string`

Specifies a `string` that contains the name of a `label`.

---

# Notes

# npm
The `name` property in the `package.json` file must be lowercase alpha-numeric
and delimited by either the dash `-` or underscore `_` characters.

__Reference__
* [npm: Creating a package JSON file](https://docs.npmjs.com/creating-a-package-json-file).

The `name` property may also include a *scope* prefix, representing the
publishing organization.

### Scope
The **scope** is preceded with an `@` symbol and appears to officially follow
the same naming rules as the `name` property (citation needed).

Scope is designated in the `package.json` as a prefix in the `name`
property, with a slash `/` character separating the scope name from the
package name. 

Example: `@my-org/my-package_name`

__Reference__
* [npm: About scopes](https://docs.npmjs.com/about-scopes)
* [npm: Creating and publishing scoped public packages](https://docs.npmjs.com/creating-and-publishing-scoped-public-packages)

When importing source-code from a scoped package via NodeJS, the full `name`
property is used as well. 

Example: `import MyClass from '@my-org/my-package_name/my/Class;`

The `node_modules` file system represents scope as a top-level directory by
its name, excluding the leading `@` symbol. Each scoped package is
installed as a sub-directory, by package name excluding the scope name prefix.

Example:  
`node_modules/asmov/bullfrog-common`  
`node_modules/asmov/bullfrog-local`

When importing without a module loader (browser), the same file
structure will have to exist in the working path. The `@` symbol
must be **included** in the scope directory name (unlike `node_modules`)
as the symbol will be taken literally as part of the path when imported.

Example:  
`asmov/bullfrog-common`  
`asmov/bullfrog-local`  
`@asmov => asmov`

To access the rest of the `sources` path, another symlink must be made.
An HTTP server will need to provides a `sources` directory outside of the URL
path and then be configured to follow this symlink.

Example:  
`/var/sites/asmov.dev/sources/js/asmov/bullfrog-common`  
`/var/sites/asmov.dev/sources/js/asmov/bullfrog-w3c`  
`/var/sites/asmov.dev/www/sources => /var/sites/asmov.dev/sources`  
`/var/sites/asmov.dev/www/@asmov => sources/js/asmov`  
`/var/sites/asmov.dev/www/index.mjs => sources/js/asmov/bullfrog-w3c/bin.w3c/index.mjs`

It appears that npm actually allows other delimiters in package and scope
names, such as the `.` character. The most common naming format among published
packages appears to use `-` as the delimiter.

#pip

__Reference__
* [python: Packing and distributing projects | setup | name](https://packaging.python.org/guides/distributing-packages-using-setuptools/#setup-name)
* [python: PEP 508 -- Dependency specification for Python Software Packages | Names](https://www.python.org/dev/peps/pep-0508/#names)
* [python: Packaging namespace packages](https://packaging.python.org/guides/packaging-namespace-packages)
* [python: The import system](https://docs.python.org/3/reference/import.html)
* [python: PEP 420 -- Implicit Namespace Packages](https://www.python.org/dev/peps/pep-0420)
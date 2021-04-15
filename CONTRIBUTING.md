# Contributing

## Guidelines

### Nameing Convention
Naming convention will follow the same convention as given in the NVMe spec.

1. Parentheical names will be preferred over full names when it is contained
   within a namespace and is more than 1 character (i.e.: `sc` over `status_code`).
1. The following words will be replaced the shortened version:

|Original|Replacement|
--- | ---
|asymmetric|asym|
|asynchronous|async|
|attribute|attr|
|command|cmd|
|controller|ctrl|
|end-to-end|e2e|
|error|err|
|firmware|fw|
|identifier|id|
|information|info|
|location|loc|
|namespace|nmsp|
|parameter|param|
|reference|ref|
|request|req|

3. Words within names which are repeated often within the namespace may be
   dropped.
1. Acronyms used within an identifier will have the first letter capitalized
   and successive letters lower case (i.e.: `InvalidSglSegmentDescriptor`
   instead of `InvalidSGLSegmentDescriptor`).

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
|available|avail|
|command|cmd|
|composite|comp|
|controller|ctrl|
|critical|crit|
|end-to-end|e2e|
|endurance|endur|
|error|err|
|firmware|fw|
|group|grp|
|identifier|id|
|information|info|
|location|loc|
|management|mgmt|
|media and data|mad|
|namespace|nmsp|
|parameter|param|
|power|pwr|
|reference|ref|
|request|req|
|threshold|thresh|
|vendor|vndr|

3. Words within names which are repeated often within the namespace may be
   dropped.
1. Acronyms used within an identifier will have the first letter capitalized
   and successive letters lower case (i.e.: `InvalidSglSegmentDescriptor`
   instead of `InvalidSGLSegmentDescriptor`).
1. Bitfields which have value as a raw integer (i.e.: critical_warning from
   SMART) should be modelled as an integer. A function should be provided with
   the same name to decode to a bitfield struct.

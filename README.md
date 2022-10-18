**Machine-readable database of public packages on crates.io which meet an
arbitrary, unwritten, sensible definition of name squatting:**

<p align="center"><b><a href="squatted.csv">squatted.csv</a></b></p>

<br>

## Format

The CSV columns are `crate,user,version`. For example:

```csv
squat,bbqsrc,0.0.1
```

None of the columns can contain commas or whitespace, so quoting or escaping are
not needed.

The <kbd>version</kbd> is the version number of the most recently published
version of the crate. This is not necessarily the largest published version
number by semver comparison, since crates.io allows publishing a smaller number
after a larger number.

The <kbd>user</kbd> is the GitHub username who published the listed version.

Only one version per crate is recorded. When a new version gets published to
crates.io, if the new version still qualifies as squatting then the version
number gets bumped in this dataset, otherwise the crate is removed from the
dataset.

<br>

## Contributing

Pull requests are welcome adding or removing entries in the CSV.

A crate must be present in the most recent nightly crates.io database dump in
order to be eligible, i.e. we do not need entries for crates already deleted by
a crates.io admin.

<br>

## License

<sup>
To the extent that it constitutes copyrightable work, the squatted.csv data file
is licensed under the
CC0 1.0 Universal license (<a href="LICENSE-CC0">LICENSE-CC0</a>)
and may be used without attribution. Anything else found in this repo is
licensed under either of
Apache License, Version 2.0 (<a href="LICENSE-APACHE">LICENSE-APACHE</a>)
or
MIT license (<a href="LICENSE-MIT">LICENSE-MIT</a>)
at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>

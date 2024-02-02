# Releases

Releases are per feature for now. When it's stable then a new release will be
created.

## Versioning

No hard rules yet. The bin and the lib follows each other. Even if there are not
changes in `todoistctl`, it'll still follow the versioning of `libtodoist`.
Later it may change, but for now they always match.

## How to create a new release

1. Update the following files:
    - `bin/todoistctl/Cargo.toml`
    - `lib/libtodoist/Cargo.toml`

2. Make sure `Cargo.lock` is up-to-date (like run `cargo check`).
3. Commit changes.
4. Create a new tag.
    ```
    ❯ git tag -s v0.3.5 -m 'v0.3.5'
    ```
5. Publish the packages.
    ```
    ❯ ./hack/scripts/publish.sh
     - Current todoistctl version:          0.3.5
     - Current libtodoist version:          0.3.5
     - libtodoist version in project file:  0.3.5

    Does this look good to you?

    Press ^C to exit or any key to continue...
    ```

6. If all version numbers are correct, press any key to continue and publish
   both the library and the binary.

### Release to Gitea (custom registry)

**Setup** (one time action)

Add registry details to your `~/.cargo/config.toml` file:

```
[registries.gitea]
index = "sparse+https://gitea.code-infection.com/api/packages/efertone/cargo/"
token = "Bearer YourFancyPAT"
```

For more details, check [Gitea's docs about cargo packages][gitea-cargo]

**Release**

The only change is the `./hack/scripts/publish.sh` call. If there is an extra
argument, it'll use that value as registry name:

```
❯ ./hack/scripts/publish.sh gitea
 - Current todoistctl version:          0.3.5
 - Current libtodoist version:          0.3.5
 - libtodoist version in project file:  0.3.5

Does this look good to you?

Press ^C to exit or any key to continue...
```

[gitea-cargo]: https://docs.gitea.com/usage/packages/cargo

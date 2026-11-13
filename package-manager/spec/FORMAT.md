# Blox Package format (`.blox`)

A `.blox` file is a ZIP archive. It contains a UTF-8 `manifest.json` at its root and application files below `payload/`.

Required manifest fields:

- `format`: `Blox Package`
- `format_version`: `1`
- `id`: stable reverse-domain or lowercase identifier
- `name`: display name
- `version`: application version
- `files`: array of `{ "path": "relative/path", "sha256": "...", "size": 123 }`

Optional fields: `publisher`, `architecture` (`any`, `x86_64`, or `aarch64`), `entrypoint`, `description`.

Example:

```json
{
  "format": "Blox Package",
  "format_version": 1,
  "id": "com.boxzero.calculator",
  "name": "BloxOS Calculator",
  "version": "1.0.0",
  "publisher": "BoxZero",
  "architecture": "any",
  "entrypoint": "bin/calculator",
  "files": [
    { "path": "bin/calculator", "sha256": "SHA256_HEX_DIGEST", "size": 1234 }
  ]
}
```

The package manager verifies file hashes and rejects absolute paths, traversal components, duplicate entries, unexpected payload files, and oversized archives. Version 1 does not execute install scripts. SHA-256 provides integrity checking, not publisher authenticity; signed packages and repository trust are future work.

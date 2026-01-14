# 8zip

## Extract

```bash
# Extract to current directory
8zip extract ./archive.tar.gz
8zip x ./archive.tar.gz

# Extract to archive directory
8zip extract --output ./archive ./archive.tar.gz

# Extract removing the first layer
8zip extract --output ./archive --strip-components 1 ./archive.tar.gz

# Streaming
cat ./archive.tar.gz | 8zip --output ./archive --format gz
cat ./archive.tar.xz | 8zip --output ./archive --format xz
cat ./archive.zip    | 8zip --output ./archive --format zip # Mimics streaming
```

## Compress

```bash
# Compress current directory (includes dotfiles)
8zip compress --output ./archive.tar.gz .
8zip c --output ./archive.tar.gz .

# Compress ./archive directory
8zip compress --output ./archive.tar.gz --cwd ./archive .
```
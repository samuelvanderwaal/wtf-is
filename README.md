# WTF Is

I was tired of looking up Metaplex program errors.

```bash
wtf-is <hex_code || decimal_code>
```

Example:

```bash
$ wtf-is 0x37
> 0x37: 
        Token Metadata            |     CannotUnverifyAnotherCreator: You cannot unilaterally unverify another creator
```

```bash
$ wtf-is 55
> 0x37: 
        Token Metadata            |     CannotUnverifyAnotherCreator: You cannot unilaterally unverify another creator
```

## Update

[Metaboss](https://github.com/samuelvanderwaal/metaboss) now supports looking up errors with `metaboss find error <code>`. Error lists in Metaboss will be kept up-to-date and this repo will be deprecated eventually.

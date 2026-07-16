# OpenMW provenance and notices

Source snapshot: `openmw-master`, OpenMW 0.52.0.

## ESM4 source notice

The adapted `components/esm4` files carry this notice (copyright years vary by
file; the original file headers remain available in the supplied snapshot):

> This software is provided 'as-is', without any express or implied warranty.
> In no event will the authors be held liable for any damages arising from the
> use of this software.
>
> Permission is granted to anyone to use this software for any purpose,
> including commercial applications, and to alter it and redistribute it
> freely, subject to the following restrictions:
>
> 1. The origin of this software must not be misrepresented; you must not claim
>    that you wrote the original software. If you use this software in a
>    product, an acknowledgment in the product documentation would be
>    appreciated but is not required.
> 2. Altered source versions must be plainly marked as such, and must not be
>    misrepresented as being the original software.
> 3. This notice may not be removed or altered from any source distribution.
>
> cc9cii, cc9c@iinet.net.au

`apps/openmw/mwworld/cellref.cpp` is part of the GPL-3.0 OpenMW package. Its
destination-cell lookup behavior was adapted into this GPL-3.0 project. The
OpenMW GPL-3.0 license text is present at `openmw-master/LICENSE`; bevyout's
GPL-3.0 license is present at the repository root.

The individual ESM4 copyright header attached to each adapted source is:

```text
components/esm4/common.hpp: Copyright (C) 2015-2020 cc9cii
components/esm4/reader.hpp: Copyright (C) 2015-2016, 2018, 2020-2021 cc9cii
components/esm4/reader.cpp: Copyright (C) 2015-2021 cc9cii
components/esm4/loadtes4.hpp: Copyright (C) 2015-2016, 2018, 2020-2021 cc9cii
components/esm4/loadtes4.cpp: Copyright (C) 2015-2016, 2018, 2020-2021 cc9cii
components/esm4/loadcell.hpp: Copyright (C) 2015-2016, 2018-2020 cc9cii
components/esm4/loadcell.cpp: Copyright (C) 2015-2016, 2018-2021 cc9cii
components/esm4/lighting.hpp: Copyright (C) 2020 cc9cii
components/esm4/loadrefr.hpp: Copyright (C) 2015-2016, 2018, 2020-2021 cc9cii
components/esm4/loadrefr.cpp: Copyright (C) 2015-2016, 2018, 2020-2021 cc9cii
components/esm4/loadachr.hpp: Copyright (C) 2016, 2018, 2020-2021 cc9cii
components/esm4/loadachr.cpp: Copyright (C) 2016, 2018, 2020-2021 cc9cii
components/esm4/loadnavm.hpp: Copyright (C) 2015, 2018, 2020 cc9cii
components/esm4/loadnavm.cpp: Copyright (C) 2015-2016, 2018, 2020-2021 cc9cii
components/esm4/loaddoor.hpp: Copyright (C) 2016, 2018, 2020 cc9cii
components/esm4/loaddoor.cpp: Copyright (C) 2016, 2018, 2021 cc9cii
components/esm4/loadcont.hpp: Copyright (C) 2016, 2018, 2020 cc9cii
components/esm4/loadcont.cpp: Copyright (C) 2016, 2018, 2021 cc9cii
components/esm4/loadnpc.hpp: Copyright (C) 2016, 2018-2021 cc9cii
components/esm4/loadnpc.cpp: Copyright (C) 2016-2021 cc9cii
components/esm4/loadcrea.hpp: Copyright (C) 2016, 2018, 2020 cc9cii
components/esm4/loadcrea.cpp: Copyright (C) 2016, 2018, 2020-2021 cc9cii
components/esm4/loadweap.hpp: Copyright (C) 2016, 2018-2020 cc9cii
components/esm4/loadweap.cpp: Copyright (C) 2016, 2018-2021 cc9cii
components/esm4/loadammo.hpp: Copyright (C) 2016, 2018-2020 cc9cii
components/esm4/loadammo.cpp: Copyright (C) 2016, 2018-2021 cc9cii
components/esm4/loadarmo.hpp: Copyright (C) 2016, 2018-2020 cc9cii
components/esm4/loadarmo.cpp: Copyright (C) 2016, 2018-2021 cc9cii
components/esm4/loadmisc.hpp: Copyright (C) 2016, 2018, 2020 cc9cii
components/esm4/loadmisc.cpp: Copyright (C) 2016, 2018, 2020-2021 cc9cii
components/esm4/loadalch.hpp: Copyright (C) 2016, 2018, 2020 cc9cii
components/esm4/loadalch.cpp: Copyright (C) 2016, 2018, 2020-2021 cc9cii
components/esm4/loadbook.hpp: Copyright (C) 2016, 2018, 2020 cc9cii
components/esm4/loadbook.cpp: Copyright (C) 2016, 2018, 2020-2021 cc9cii
components/esm4/loadkeym.hpp: Copyright (C) 2016, 2018, 2020 cc9cii
components/esm4/loadkeym.cpp: Copyright (C) 2016, 2018, 2020-2021 cc9cii
components/esm4/loadnote.hpp: Copyright (C) 2019, 2020 cc9cii
components/esm4/loadnote.cpp: Copyright (C) 2019-2021 cc9cii
components/esm4/loadligh.hpp: Copyright (C) 2016, 2018-2020 cc9cii
components/esm4/loadligh.cpp: Copyright (C) 2016, 2018, 2020-2021 cc9cii
components/esm4/loadacti.hpp: Copyright (C) 2016, 2018, 2020 cc9cii
components/esm4/loadacti.cpp: Copyright (C) 2016, 2018, 2020-2021 cc9cii
components/esm4/loadtact.hpp: Copyright (C) 2019, 2020 cc9cii
components/esm4/loadtact.cpp: Copyright (C) 2019-2021 cc9cii
components/esm4/loadterm.hpp: Copyright (C) 2019, 2020 cc9cii
components/esm4/loadterm.cpp: Copyright (C) 2019-2021 cc9cii
components/esm4/loadaspc.hpp: Copyright (C) 2020 cc9cii
components/esm4/loadaspc.cpp: Copyright (C) 2020 cc9cii
components/esm4/loadlgtm.hpp: Copyright (C) 2020 cc9cii
components/esm4/loadlgtm.cpp: Copyright (C) 2020-2021 cc9cii
components/esm4/loadmusc.hpp: Copyright (C) 2020 cc9cii
components/esm4/loadmusc.cpp: Copyright (C) 2020 cc9cii
components/esm4/loadsndr.hpp: Copyright (C) 2020 cc9cii
components/esm4/loadsndr.cpp: Copyright (C) 2020 cc9cii
components/esm4/loadsoun.hpp: Copyright (C) 2016, 2018, 2020 cc9cii
components/esm4/loadsoun.cpp: Copyright (C) 2016, 2018, 2020 cc9cii
```

`apps/openmw/mwworld/cellref.cpp` has no per-file header in this snapshot; its
license is inherited from the GPL-3.0 package.

## Source map and SHA-256

Each entry is relative to `openmw-master`:

```text
components/esm4/common.hpp af56ec8b62777109159f62ac7a0fc45135460a122a1dccf78825a2fb0a2c7493
components/esm4/reader.hpp 6c4624ef63dee35bd630afd4e7f0d618285deb35859ef380eb76bd5f0930f2ca
components/esm4/reader.cpp 11f77b590d031f1e68735becc7f0b70abc149e68a88b862078500ad17948d871
components/esm4/loadtes4.hpp cc0edd81f5a665bfe9ed51faede01089e8860ebd7d94fe6c6b7ccfe690f61662
components/esm4/loadtes4.cpp 38e80194a30c258bc614431aabb44e9542e679cbea9083340567fc1ffdc53c76
components/esm4/loadcell.hpp f78d161a05e230eb9edfde5a8b79351c2be3ce9aca364acc8c7a6f52e51cfbfb
components/esm4/loadcell.cpp 2c034c0e9232ebd79b0507a6c907e5ee7f62cce3801d1bd79582382818180b93
components/esm4/lighting.hpp 3509b0c95f6568d070b5c6bd0c6809bc4aac6e558ebdaf5190b0902656dc74ba
components/esm4/loadrefr.hpp 989f4bb961b7d2a06d2b282157423955d495b2227ea78114856973fe8c27c334
components/esm4/loadrefr.cpp f772bbc5df979ba18287f72c8b32b8c624691c957f231a8e7f15fd3cc93e242a
components/esm4/loadachr.hpp 798b4477eacc3c16de1b7273113604649c0d9eecdd31937c0cc36d56631c051c
components/esm4/loadachr.cpp ef797a31885a0c2ac65a00413021dc6d7b50588790af534bc32d8bafefacc228
components/esm4/loadnavm.hpp 9362e1515670477672a28d42292353dd1d815deca25a9dad37ec29b00ae268cb
components/esm4/loadnavm.cpp 213d38f56f28e02aed409a67a339d0ac275a3ad5776373577249a7c0e5ebfafa
components/esm4/loaddoor.hpp f05f7e43888482998ac8f18112d574156a4bebda9397163d0ee6fe8c4ab6e98e
components/esm4/loaddoor.cpp 6fe2b92c6ae43df7ce0afb71eae140c9dbd0d347ed0907c4559420e5b558b30b
components/esm4/loadcont.hpp 02d858a2a8419e46d9d2a6baea7589aae63354efc42a8469f07cce9f6e57d832
components/esm4/loadcont.cpp a12c4da7c316bc0ac7f19d81277532f52296cb63ab82f0eb913dd6959eb9d0da
components/esm4/loadnpc.hpp 04a9f2a7993b18d91b4daf6ce5a367cb9b3304edacd9bdeeec52769b74a7d0cb
components/esm4/loadnpc.cpp 413dbcd27c081925d6d359e02388f75339528781112e445760f95509e8c9ae18
components/esm4/loadcrea.hpp d62a05dc6dd9a3ae842e5c5adf1c60b010cfa20a37e91db823dc7b348983eaab
components/esm4/loadcrea.cpp b17ef926fbeff4e35d84f215973135c2450492a7a6103480ee6e43ebed4897ca
components/esm4/loadweap.hpp e5c906427609f7ac7b14724354f73c7f8b2944672b55705b3546046a87807e65
components/esm4/loadweap.cpp 8a5880b25cd83e9b4e8612d75543c8331941028ebd2b99df1561e70320b5e312
components/esm4/loadammo.hpp 2afeea92f71357a850d65a3b0f8302a651173a51a3417f2368dfc187f0955503
components/esm4/loadammo.cpp 96040f1d560ac0ee0f369926190379c382094321d501f69af82fa90cbcabb4b0
components/esm4/loadarmo.hpp af7cf06a016d11d77ec2ae589cd8fd0a300a1cecfd4f5eb061b3651d409270c8
components/esm4/loadarmo.cpp aaff7811b23ea4b6306e8346e5fb3e2636bd6623c17e98b8bc9bdcf50270f4fd
components/esm4/loadmisc.hpp 906e4283685a8930c8255069f1d254f0f54075b6b7d69448e67f56951761ad69
components/esm4/loadmisc.cpp 1f89b0f7e271bf2da1b9cbf8cb800a05f861c4502601ca9cfa29395ab33a9e2a
components/esm4/loadalch.hpp f10dfec367bc6e06fa715e8f7cc5240d9c5e7e01793fdc5c7aba7cb09dbea4a9
components/esm4/loadalch.cpp 3433732ebd4fe708a54027bf79dbefcd04ebf2e4f04b4add2aa4f9ba25fb9c53
components/esm4/loadbook.hpp 25e32baa056c0b02964ba753cb2eea46060cafbf44b49230a2b98b5f7dbd052e
components/esm4/loadbook.cpp 42e5e33b159e8cd138424355970ad208246dceac315be829081f62c4f7792a17
components/esm4/loadkeym.hpp 56a614657614a3f168d153108121ac64100df32af47b06f4847e5ec494d2e510
components/esm4/loadkeym.cpp ccb31c3c16ede25fc5eb4d4cbd5381c50c52f52cc13f5c58c8f98238421f7715
components/esm4/loadnote.hpp 9a7ad2394818ebc7479b2414483c0ad3ccce3fb3a4b056c9c3951e8861456761
components/esm4/loadnote.cpp 0fa2245e9bf7502d3ed90a1e0dd96c735c57e44e95045af07eda9d509a7fa475
components/esm4/loadligh.hpp a36cfc8c4793193bcdfe6bb86bc40918ee57b1f35751b4df2c0f7533325d88e3
components/esm4/loadligh.cpp d505b8706e9ed9b6207dbcfaa7f046200f3784e55de5216ee32f2ad4cb4674d7
components/esm4/loadacti.hpp 0defb3eb9ae7698dc4a7c5b04563d58c78ce8bc3126c7c8ca193100033ba04fa
components/esm4/loadacti.cpp 0a535db71da0c5e2122c2dc2a8d08163755c521637bf0ead04795e53ab9690b0
components/esm4/loadtact.hpp 8e973f572b3706a5ed9ab497879839598315de2c69147647197a8cf169b55796
components/esm4/loadtact.cpp 403adc81a363620f0070c59a85c2d369d9b4570981c2e57a93c7dd4226da2d2c
components/esm4/loadterm.hpp b9c7e1eba9c0a4d55d0b289d52c309901dd19545ffcd62f3608596fa5404c05a
components/esm4/loadterm.cpp 546eeb2f1c2daade0043bc474be055f9976930656fa1519a42d698bedb7c47b2
components/esm4/loadaspc.hpp 505ed1904d4aae359a3000ae392411c78417b6f107ea3a4ae1596193963a796a
components/esm4/loadaspc.cpp 97bb6f8e28302b16aaaefdf1824d36bd6c7342067f820ae59bc4c7e50fa8669b
components/esm4/loadlgtm.hpp 427c830f60fe190588b1a89525b4db616e397d579b3b365a458a20ed9c17e005
components/esm4/loadlgtm.cpp dffca5a2dd5ad192f6d81c864bc07f01c841c0d82f13dbe2af191c2e43b8ac23
components/esm4/loadmusc.hpp 9d19d3bc83244fca2f975624588d5ae941ee621ea1a85448549ba16ade4f5edf
components/esm4/loadmusc.cpp ad83d548a483142fee69d1dc2b2dc74832f3d808fd51a7c0f202401c93715b7d
components/esm4/loadsndr.hpp a18345e9423cbaf8f26b6689c82a50b16f7eaf4347278fff2187fdc84f554cda
components/esm4/loadsndr.cpp 9f96cfe667ddeda84310ac071b7821a01a91caf698374104f8d63746d5a20925
components/esm4/loadsoun.hpp 4bf8c12e8c786e36933b10d1cbcf3e96c0bb6882127e411c2da548ebc1b402cf
components/esm4/loadsoun.cpp a86b6d2c16cf7a4a2dae22f6b09e9363b09940d6d88043eebde74d403477e880
apps/openmw/mwworld/cellref.cpp 7c9e7857de758b40c93e0622d3256972fed478ec1b1c43b339a93f1afe343f98
```

## Adaptation notes

- C++ reader/store types were replaced with owned Rust data and `anyhow`
  errors.
- Unknown record and subrecord types are ignored or represented semantically;
  malformed size boundaries remain hard errors.
- The original runtime object hierarchy was not copied.
- FO3 NAVM chunks are catalogued and retained, not decoded.
- Sound, sound-descriptor, acoustic-space, music, lighting-template, activator,
  terminal, and tactical-activator fields were ported as owned metadata for
  the audio/world-state milestone. Their source files are listed above; the
  original headers identify cc9cii with the per-file copyright years.
- The inventory catalog adaptation uses the listed `loadweap`, `loadarmo`,
  `loadammo`, `loadalch`, `loadmisc`, `loadbook`, `loadnote`, and `loadkeym`
  files for record sizes, value/weight fields, icon paths, condition, text,
  and effect FormIDs. FO3-only `ARMO.DNAM` DR and `AMMO.DNAM` damage decoding
  is an explicitly marked extension where the snapshot retains or skips the
  subrecord.
- The `RCPE` recipe adapter is a Fallout 3/New Vegas ESM4 extension: the
  supplied OpenMW snapshot has no dedicated RCPE loader. Its Rust boundary
  follows the documented 16-byte `DATA` layout and preserves `CTDA` bytes
  opaquely; malformed pairs remain diagnostics and are never converted into
  invented recipe data.
- `lighting.hpp` supplies the FO3 40-byte `XCLL`/`LGTM.DATA` layout; the
  `loadcell` and `loadlgtm` adaptations preserve the nine named `LNAM`
  inheritance masks while leaving unresolved templates on CELL lighting.

## Adapted File Contributors

Upstream Git history identifies the following contributors for these source files:

- Alexei Dobrohotov
- Alexei Kotov
- Andrei Kortunov
- Andrzej Głuszak
- AnyOldName3
- Austin English
- Bret Curtis
- Cédric Mocquillon
- Capostrophic
- cc9cii
- dteviot
- elsid
- Evil Eye
- florent.teppe
- fteppe
- jvoisin
- Kindi
- mrohrlach
- Petr Mikheev
- Project579
- psi29a
- scrawl
- Shi Han
- Zackhasacat

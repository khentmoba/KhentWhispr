# Third-party notices

SpeakFlow contains or interoperates with third-party software and assets.
The project license in [`LICENSE`](LICENSE) applies only to material for which
the SpeakFlow licensor can grant those rights. The items below remain under
their own terms.

## FireRedVAD model assets

The bundled upstream FireRedVAD ONNX model and CMVN data, together with portions
of the integration adapted from upstream, remain subject to the Apache License
2.0. The license text is available at
[`src-tauri/resources/FireRedVAD-LICENSE.txt`](src-tauri/resources/FireRedVAD-LICENSE.txt).
SpeakFlow's original integration and post-processing changes are licensed
under GPL-3.0-only.

## transcribe.cpp and downloaded models

[transcribe.cpp](https://github.com/handy-computer/transcribe.cpp) is distributed
under the MIT License. Qwen3-ASR, Parakeet and Canary-Qwen model files are
downloaded separately and remain subject to the terms published by their
respective upstream providers.

Package-manager dependencies may carry additional licenses. Their inclusion in
the source tree, Python engine, or application bundle does not change those
licenses.

## Parakeet models

Parakeet Unified EN 0.6B and Parakeet TDT 0.6B v2 are downloaded separately as
Q8_0 GGUF ports published by
[handy-computer](https://huggingface.co/handy-computer) for transcribe.cpp. They
derive from NVIDIA models and are not redistributed in SpeakFlow's installers.

- [Parakeet Unified EN 0.6B GGUF](https://huggingface.co/handy-computer/parakeet-unified-en-0.6b-gguf)
  is governed by the NVIDIA Open Model License, as stated by the
  [upstream NVIDIA model card](https://huggingface.co/nvidia/parakeet-unified-en-0.6b)
  and the GGUF repository metadata. The transcribe.cpp documentation states
  CC-BY-4.0, which does not match the NVIDIA card. Review the NVIDIA license
  before redistributing these weights.
- [Parakeet TDT 0.6B v2 GGUF](https://huggingface.co/handy-computer/parakeet-tdt-0.6b-v2-gguf)
  is licensed CC-BY-4.0, as stated by the
  [upstream NVIDIA model card](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v2).

## Qwen3-ASR 1.7B model

The [Qwen3-ASR 1.7B GGUF](https://huggingface.co/handy-computer/Qwen3-ASR-1.7B-gguf)
is downloaded separately and its repository metadata states Apache-2.0.

## Canary-Qwen 2.5B model

The [Canary-Qwen 2.5B GGUF](https://huggingface.co/handy-computer/canary-qwen-2.5b-gguf)
is downloaded separately. It is a GGUF port, published by handy-computer, of
[NVIDIA Canary-Qwen 2.5B](https://huggingface.co/nvidia/canary-qwen-2.5b). The
upstream model is licensed under
[CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/legalcode.en), and its
repository metadata states the same license.

## Confucius4-R2T2 native runtime and model

[audio.cpp](https://github.com/0xShug0/audio.cpp), copyright 2026 ShugoAI LLC,
is distributed under Apache-2.0. The build pins revision
`6c70f32d0d90a29a9863e556bd9712ce622f868b` and applies the reviewed patch in
`scripts/patches/audio-cpp-r2t2-windows-streaming.patch` for rolling streaming
and empty-prefix handling. Runtime bundles retain the audio.cpp, ggml and
SentencePiece license notices alongside their DLLs. The CUDA bundle also
includes NVIDIA's redistribution license; Microsoft runtime DLLs retain
their vendor terms.

The rolling inference adaptation also follows NetEase Youdao's R2T2 reference
implementation, whose Apache-2.0 license is retained in
[`R2T2-CODE-LICENSE.txt`](src-tauri/resources/R2T2-CODE-LICENSE.txt).

The separately downloaded [Q8_0 GGUF](https://huggingface.co/davidxifeng/Confucius4-R2T2-gguf)
is derived from [NetEase Youdao Confucius4-R2T2](https://github.com/netease-youdao/Confucius4-R2T2).
It is subject to the NetEase Youdao Model Use License Agreement, not this
project's GPL license. A copy is bundled as
[`R2T2-MODEL-LICENSE.txt`](src-tauri/resources/R2T2-MODEL-LICENSE.txt).

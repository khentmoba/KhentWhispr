import os
import sys
import unittest
from unittest import mock

import numpy as np

sys.path.insert(0, os.path.dirname(__file__))

from firered_vad import FireRedVad, FireRedVadOptions


class FireRedVadPostprocessingTests(unittest.TestCase):
    def _vad(self, **overrides):
        vad = object.__new__(FireRedVad)
        options = {
            "smooth_window_frames": 1,
            "min_speech_duration_ms": 100,
            "min_silence_duration_ms": 200,
            "speech_pad_ms": 0,
        }
        options.update(overrides)
        vad.options = FireRedVadOptions(**options)
        return vad

    def test_rejects_short_probability_spike(self):
        probabilities = np.concatenate(
            [np.zeros(10), np.ones(9), np.zeros(30)]
        ).astype(np.float32)

        self.assertEqual(
            self._vad()._timestamps_from_probabilities(probabilities, 8_000),
            [],
        )

    def test_emits_and_merges_padded_speech_segments(self):
        vad = self._vad(speech_pad_ms=120)
        probabilities = np.concatenate(
            [
                np.zeros(20),
                np.ones(15),
                np.zeros(20),
                np.ones(15),
                np.zeros(30),
            ]
        ).astype(np.float32)

        self.assertEqual(
            vad._timestamps_from_probabilities(probabilities, 16_000),
            [{"start": 1_280, "end": 13_120}],
        )

    def test_default_options_preserve_short_utterances_and_wider_consonant_padding(self):
        vad = object.__new__(FireRedVad)
        vad.options = FireRedVadOptions()
        probabilities = np.concatenate(
            [np.zeros(30), np.ones(12), np.zeros(40)]
        ).astype(np.float32)

        self.assertEqual(
            vad._timestamps_from_probabilities(probabilities, 16_000),
            [{"start": 1_920, "end": 10_240}],
        )


class FireRedVadAssetTests(unittest.TestCase):
    def test_thread_budget_respects_single_core_and_unknown_cpu_count(self):
        for cores, expected in [(1, 1), (None, 1), (16, 2)]:
            with self.subTest(cores=cores), mock.patch("firered_vad.os.cpu_count", return_value=cores):
                vad = FireRedVad()
                self.assertEqual(vad._session.get_session_options().intra_op_num_threads, expected)
                self.assertEqual(vad._session.get_session_options().inter_op_num_threads, 1)

    def test_two_threads_preserve_model_probabilities_and_boundaries(self):
        baseline = FireRedVad(intra_op_num_threads=1)
        candidate = FireRedVad(intra_op_num_threads=2)
        rng = np.random.default_rng(42)
        audio = np.concatenate([
            np.zeros(16000, dtype=np.float32),
            rng.standard_normal(32000).astype(np.float32) * 0.1,
            np.zeros(16000, dtype=np.float32),
        ])
        np.testing.assert_array_equal(baseline.probabilities(audio), candidate.probabilities(audio))
        self.assertEqual(baseline.speech_timestamps(audio), candidate.speech_timestamps(audio))

    def test_bundled_model_rejects_one_second_of_silence(self):
        vad = FireRedVad()
        probabilities = vad.probabilities(np.zeros(16_000, dtype=np.float32))

        self.assertEqual(probabilities.shape, (98,))
        self.assertEqual(vad.speech_timestamps(np.zeros(16_000, dtype=np.float32)), [])


if __name__ == "__main__":
    unittest.main()

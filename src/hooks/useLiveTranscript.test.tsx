import { act, renderHook } from "@testing-library/react";
import { beforeEach, expect, it } from "vitest";
import { LIVE_TRANSCRIPT_KEY, useLiveTranscript } from "./useLiveTranscript";

beforeEach(() => localStorage.clear());

it("saves the toggle, updates mounted consumers, and restores it after remount", () => {
  const first = renderHook(() => useLiveTranscript());
  const second = renderHook(() => useLiveTranscript());
  expect(first.result.current[0]).toBe(true);
  act(() => first.result.current[1](false));
  expect(localStorage.getItem(LIVE_TRANSCRIPT_KEY)).toBe("false");
  expect(second.result.current[0]).toBe(false);
  first.unmount();
  const restored = renderHook(() => useLiveTranscript());
  expect(restored.result.current[0]).toBe(false);
});

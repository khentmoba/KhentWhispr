import { fireEvent, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import {
  SelectionToolbar,
  type SelectionToolbarAction,
} from "../SelectionToolbar";

function renderToolbar(overrides: Partial<Parameters<typeof SelectionToolbar>[0]> = {}) {
  const onAction = vi.fn<(action: SelectionToolbarAction) => void>();
  const onStartDrag = vi.fn();
  const props = {
    selectionText: "selected text",
    onAction,
    onStartDrag,
    ...overrides,
  };

  return { ...render(<SelectionToolbar {...props} />), ...props };
}

describe("SelectionToolbar", () => {
  it("exposes an accessible toolbar", () => {
    renderToolbar();

    expect(screen.getByRole("toolbar", { name: "Selection assistant" })).toBeInTheDocument();
  });

  it.each(["Translate", "Explain", "Improve", "Copy", "Search"])(
    "renders a visible %s action label",
    (name) => {
      renderToolbar();

      const button = screen.getByRole("button", { name });
      expect(button).toHaveTextContent(name);
    },
  );

  it("lets the explicit close button dismiss the toolbar", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    renderToolbar({ onClose });

    await user.click(screen.getByRole("button", { name: "Close" }));

    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it.each([
    ["Translate", "translate"],
    ["Explain", "explain"],
    ["Improve", "optimize"],
    ["Copy", "copy"],
    ["Search", "search"],
  ] as const)("dispatches %s as %s", async (label, action) => {
    const user = userEvent.setup();
    const { onAction } = renderToolbar();

    await user.click(screen.getByRole("button", { name: label }));

    expect(onAction).toHaveBeenCalledWith(action);
  });

  it("keeps actions out of the keyboard focus order", () => {
    renderToolbar();
    const button = screen.getByRole("button", { name: "Translate" });

    expect(button).toHaveAttribute("tabindex", "-1");
  });

  it("isolates its own pointer events from the outside-click dismiss layer", () => {
    const outsidePointerDown = vi.fn();
    const { container } = render(
      <div onPointerDown={outsidePointerDown}>
        <SelectionToolbar
          selectionText="selected text"
          onAction={vi.fn()}
          onStartDrag={vi.fn()}
        />
      </div>,
    );

    fireEvent.pointerDown(screen.getByRole("button", { name: "Copy" }));

    expect(container).toBeTruthy();
    expect(outsidePointerDown).not.toHaveBeenCalled();
  });

  it("disables every toolbar action for a normalized empty selection", () => {
    renderToolbar({ selectionText: "  \r\n " });

    for (const name of ["Translate", "Explain", "Improve", "Copy", "Search"]) {
      expect(screen.getByRole("button", { name })).toBeDisabled();
    }
  });

  it("keeps screenshot capture out of the action toolbar", () => {
    renderToolbar();

    expect(screen.queryByRole("button", { name: "Screenshot assistance" })).not.toBeInTheDocument();
  });
});

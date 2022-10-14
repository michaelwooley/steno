import { render, screen } from "@testing-library/svelte";
import Index from "./+page.svelte";

describe("Test index.svelte", () => {
  it("h1 exists", () => {
    const { getByText } = render(Index);
    expect(getByText("Welcome to Tauri!")).toBeTruthy();
  });
  it("link to svelte website", () => {
    render(Index);

    const link = screen.getAllByRole("link")[0];
    expect(link).toHaveAttribute("href", "https://vitejs.dev");
  });
});

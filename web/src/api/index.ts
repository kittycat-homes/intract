import { Configuration } from "@/swagger";

// is this production or dev?
function is_dev(): boolean {
  return process.env.NODE_ENV === "development";
}

export function conf(): Configuration {
  return new Configuration({
    // when to send cookies and when not to
    credentials: is_dev() ? "include" : "same-origin",
    // what url to send requests to
    basePath: is_dev() ? "http://localhost:3000" : window.location.origin,
  });
}

import { Configuration } from "@/swagger";

export function conf(): Configuration {
  return new Configuration({
    basePath:
      process.env.NODE_ENV === "development"
        ? "http://localhost:3000"
        : window.location.origin,
  });
}

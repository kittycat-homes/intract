import { Configuration } from "@/swagger";

export const conf = new Configuration({
  basePath:
    process.env.NODE_ENV === "development"
      ? "http://localhost:3000"
      : window.location.origin,
});

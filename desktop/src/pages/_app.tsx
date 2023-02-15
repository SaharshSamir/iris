import type { AppProps } from "next/app";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api";

import "../style.css";
import "../App.css";

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
	return <Component {...pageProps} />;
}

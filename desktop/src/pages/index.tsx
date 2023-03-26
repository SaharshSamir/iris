import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/shell";
import Image from "next/image";
import reactLogo from "../assets/react.svg";
import tauriLogo from "../assets/tauri.svg";
import nextLogo from "../assets/next.svg";
import axios from "axios";
// import { window } from "@tauri-apps/api";
import {Procedures} from "@iris/iris_core/bindings";
 
let test: Procedures | null = null;

let axiosInstance = axios.create({
        withCredentials: true
    });

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");

	// useEffect(() => {
	// 	invoke("init_spotlight_behaviour");
	// }, []);

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		setGreetMsg(await invoke("new_example", { name }));
	}

	async function google_oauth() {
		await open("http://localhost:1420/login");
		console.log("window location: ");
		if (window) {
			console.log(window.location);
		}
		// invoke("google_oauth");
	}

    function handleTheseNuts(){
        console.log("No redirects, just cookie");
        axiosInstance.get("/api/redirect").then(res => console.log("recvd resp", res)).catch(err => console.log(err));
        setName("what");
    }

    function testCookies(){
        axiosInstance.get("/api/cookietest").then(res => console.log(res)).catch(err => console.log(err));
    }
	return (
		<div className="container">
			<h1>Welcome to Tauri!</h1>

			<button onClick={handleTheseNuts}>Login with Dickcord</button>
			<button onClick={testCookies}>Test Cookies</button>

			<div className="row">
				<div>
					<input
						id="greet-input"
						onChange={(e) => setName(e.currentTarget.value)}
						placeholder="Enter a name..."
					/>
					<button type="button" onClick={() => greet()}>
						Greet
					</button>
				</div>
			</div>

			<p>{greetMsg}</p>
		</div>
	);
}

export default App;

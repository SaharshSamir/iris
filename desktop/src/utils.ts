import { invoke } from "@tauri-apps/api/tauri";
import { URLSearchParams } from "next/dist/compiled/@edge-runtime/primitives/url";
import {QueryClient} from "@tanstack/react-query";
import {FetchTransport, createClient} from "@rspc/client";
import {createReactQueryHooks} from "@rspc/react";
import type {Procedures} from "@iris/iris_core/bindings";

let rootUrl = "https://accounts.google.com/o/oauth2/v2/auth";
let serverUrl = "http://127.0.0.1:6969";

function getGoogleOauthUrl() {

    const options = {
        redirect_uri: "http://localhost:6969/auth/google",
        client_id: "899470897400-4qubqe3mancekjuj7a5hfq2acn0ecbmf.apps.googleusercontent.com",
        access_type: "offline",
        response_type: "code", 
        prompt: "consent",
        scope: [
            "https://www.googleapis.com/auth/userinfo.profile",
            "https://www.googleapis.com/auth/userinfo.email",
        ].join(" "),
    };
    const querySearch = new URLSearchParams(options);
     
    return `${rootUrl}?${querySearch.toString()}`;
}

export {getGoogleOauthUrl};

export type DeviceInfo = {
    name?: string,
    device_type?: "Mobile" | "Desktop"
}

export async function get_device_info(): Promise<DeviceInfo>{
    let res: string = await invoke("read_device_info");
    let info: DeviceInfo = JSON.parse(res); 
    console.log("device info in forntend: ", info);
    return info;
}

//rspc stuff
export const client = createClient<Procedures>({
    transport: new FetchTransport(`${serverUrl}/rspc`)
});

export const queryClient = new QueryClient();
export const rspc = createReactQueryHooks<Procedures>();



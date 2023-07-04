import { invoke } from "@tauri-apps/api/tauri";
import { URLSearchParams } from "next/dist/compiled/@edge-runtime/primitives/url";
import {QueryClient} from "@tanstack/react-query";
import {FetchTransport, createClient} from "@rspc/client";
import {createReactQueryHooks} from "@rspc/react";
import type {Procedures} from "@iris/iris_core/bindings";
import axios, { HeadersDefaults } from 'axios';
import { URL } from "url";

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
    console.log("device_info response", res);
    let info: DeviceInfo = JSON.parse(res); 
    console.log("device info in forntend: ", info);
    return info;
}

type Headers = {
    'Content-Type': string;
    Accept: string;
    Authorization: string | undefined;
}

//defining my own fetch transport with custom headers
function fetch_transport(input: RequestInfo | URL, init?: RequestInit | undefined):  Promise<Response> {
    let jwt = localStorage.getItem("jwt");


    return fetch(input, {
        ...init,
        headers: {
            'Content-Type': 'application/json',
            Accept: 'application/json',
            "Authorization": jwt ? jwt : undefined
        } as HeadersInit
    });
}

//rspc stuff
export const client = createClient<Procedures>({
    transport: new FetchTransport(`${serverUrl}/rspc`, fetch_transport)
});


//axiosInstance
const axiosClient = axios.create();
axiosClient.defaults.baseURL = 'http://127.0.0.1:6969';

axiosClient.defaults.headers = {
  'Content-Type': 'application/json',
  Accept: 'application/json'
} as Headers & HeadersDefaults & { [key: string]: any; }; 

axiosClient.interceptors.request.use(config => {
    const token = localStorage.getItem("jwt");
    if(token) {
        config.headers.set("Authorization", token);
    }
    return config;
})

export const queryClient = new QueryClient();
export const rspc = createReactQueryHooks<Procedures>();

type ValidationResult = {
    valid: boolean;
    message: string | undefined;
}

export function validateUsername(username: string): ValidationResult {
    let result: ValidationResult = {
        valid: true,
        message: undefined
    }

    const specialCharRegex = /[`!@#$%^&*()+\-=\[\]{};':"\\|,<>\/?~]+/;

    //can't contain space
    if(username.includes(" ")){
        result.valid = false
        result.message = "no space allowed";
    }
    //can't contain special characters
    if(specialCharRegex.test(username)) {
        result.valid = false
        result.message = "no special characters allowed";
    }


    return result;
}

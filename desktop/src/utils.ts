import { URLSearchParams } from "next/dist/compiled/@edge-runtime/primitives/url";

let rootUrl = "https://accounts.google.com/o/oauth2/v2/auth";

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

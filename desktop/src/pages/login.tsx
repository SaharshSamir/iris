import { CredentialResponse, GoogleLogin } from "@react-oauth/google";
import jwt_decode from "jwt-decode";
import {getGoogleOauthUrl} from "../utils";

const Login = () => {

    //const handleSuccess = (creds: CredentialResponse) => {
    //    const user = jwt_decode(creds.credential as string);
    //    console.log(user);
    //}

    async function handleTheseNuts(){
        console.log("did this event work?");
        await fetch("/api/redirect");
    }
	return (
		<div>
            <button onClick={handleTheseNuts}>Go To TestPage</button>
		</div>
	);
};

export default Login;

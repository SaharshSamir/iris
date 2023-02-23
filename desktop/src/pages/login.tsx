import { CredentialResponse, GoogleLogin } from "@react-oauth/google";
import jwt_decode from "jwt-decode";

const Login = () => {

    const handleSuccess = (creds: CredentialResponse) => {
        const user = jwt_decode(creds.credential);
        console.log(user);
    }

	return (
		<div>
			<GoogleLogin onSuccess={handleSuccess} />
		</div>
	);
};

export default Login;

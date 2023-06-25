import { useEffect } from "react";
import { useRouter } from "next/router";

const Index = () => {
	const router = useRouter();
	useEffect(() => {
        let jwt = localStorage.getItem("jwt");
        if(jwt){
            router.push("/home");
        }else {
            router.push("/login");
        }
	}, []);
	return <></>;
};

export default Index;

import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import BasicLayout from "../layouts/basicLayout";
import { get_device_info, rspc } from "../utils";

const Home = () => {
	let { data, isLoading } = rspc.useQuery(["getUser"]);
	let [isOnboarding, setIsOnboarding] = useState(false);
	let { push } = useRouter();

	useEffect(() => {
		get_device_info()
			.then((device_info) => {
				//if device info is not set, render the onobarding page
				if (device_info.name === undefined) {
					setIsOnboarding(true);
				}
			})
			.catch((e) => {
				console.log("some error", e);
				push("/deviceonboarding");
			});
	}, []);
	if (isLoading) {
		return <BasicLayout>"Loading..."</BasicLayout>;
	}

	if (isOnboarding) {
		console.log("pushing");
		push("/deviceonboarding");
	}
	return (
		<BasicLayout>
			<p className="break-words">{JSON.stringify(data)}</p>
		</BasicLayout>
	);
};

export default Home;
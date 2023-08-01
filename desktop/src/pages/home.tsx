import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import BasicLayout from "../layouts/basicLayout";
import { get_device_info, rspc } from "../utils";
import Header from "@components/header";
import { Device } from "@iris/iris_core/bindings";

function Mydevices(data: { devices: Device[] | undefined }) {
	return (
		<div className="bg-[#212634] mt-6 w-full rounded-md h-28">
			<p>{JSON.stringify(data.devices)}</p>
			<p className="">My Devices</p>
		</div>
	);
}

const Home = () => {
	let { data, isLoading } = rspc.useQuery(["user.getUser"]);
	let [isOnboarding, setIsOnboarding] = useState(false);
	let { push, reload } = useRouter();
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

	useEffect(() => {
		if (localStorage.getItem("jwt") === null) {
			push("login");
		}
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
			{/* <p className="break-words">{JSON.stringify(data)}</p> */}
			<Mydevices devices={data?.devices} />
		</BasicLayout>
	);
};

export default Home;

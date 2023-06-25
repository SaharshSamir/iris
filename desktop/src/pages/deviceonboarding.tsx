import { useEffect, useRef, useState } from "react";
import BasicLayout from "../layouts/basicLayout";
import { invoke } from "@tauri-apps/api";
import { useRouter } from "next/router";
import { get_device_info, rspc } from "../utils";

const DeviceOnboarding = () => {
	const inputRef = useRef<HTMLInputElement | null>(null);
	const [isOnboarding, setIsOnboarding] = useState(false);
	const router = useRouter();
	const { mutate, data } = rspc.useMutation(["addDevice"]);

	useEffect(() => {
		get_device_info().then((device_info) => {
			//if device info is not set, render the onobarding page
			if (device_info.name !== undefined) {
				// setIsOnboarding(true);
				router.push("/home");
			}
		});
	}, []);

	const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();

		if (inputRef.current && inputRef.current.value) {
			console.log(inputRef.current.value);
			await invoke("set_device_info", { deviceName: inputRef.current.value });
			mutate({
				device_type: "Computer",
				name: inputRef.current.value,
				user_id: "",
			});
			router.push("/home");
		}
	};
	return (
		<BasicLayout>
			<div className="flex flex-col  h-80 items-center">
				{/* <p className="text-3xl m-2">Device Name</p>
				<p className="">What would you like to call this device?</p> */}
				<form
					className="flex flex-col w-full justify-center items-center h-full"
					onSubmit={onSubmit}
				>
					<label>What would you like to call this device?</label>
					<input
						className="input-bordered text-slate-900 input-sm input w-2/6 mt-4 mb-2 active:decoration-none"
						placeholder="Device Name"
						type="text"
						name="deviceName"
						ref={inputRef}
					/>
					<button className="bg-purple-700 mt-3 text-sm" type="submit">
						Submit
					</button>
				</form>
			</div>
		</BasicLayout>
	);
};

export default DeviceOnboarding;

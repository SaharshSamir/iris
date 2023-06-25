import { useEffect } from "react";
import Image from "next/image";
import axios from "axios";
import { useForm, SubmitHandler } from "react-hook-form";
import { ErrorMessage } from "@hookform/error-message";
// import { window } from "@tauri-apps/api";
import { Procedures } from "@iris/iris_core/bindings";
// import Logo from "../assets/"
import Logo from "../assets/Iconex/Logo-64.png";
import { DeviceInfo, get_device_info, rspc } from "../utils";
import { useRouter } from "next/router";

let test: Procedures | null = null;


type LoginData = {
	email?: string | undefined;
	password?: string | undefined;
};


function Login() {
	const { push } = useRouter();
	const { mutate, data } = rspc.useMutation(["login"]);
	const {
		register,
		handleSubmit,
		formState: { errors },
	} = useForm<LoginData>({
		criteriaMode: "all",
	});

	const onSubmit: SubmitHandler<LoginData> = (data) => {
		console.log(data);
		//post data
		mutate({
			email: data.email || "",
			password: data.password || "",
		});
	};
	console.log("user created: ", data);
	console.log(errors);

	if (typeof data !== "undefined") {
        console.log(data);
        localStorage.setItem("jwt", data);
		push("/home");
	}

	return (
		<div className="overflow-y-auto h-screen p-5 flex flex-col bg-gradient-to-t from-[#0D1027] to-[#071E42]">
			<div className=" flex flex-col  items-center">
				<Image src={Logo} height={64} width={64} alt="logo" />
				<p className="text-4xl font-Redrose">Sign In</p>
				<form
					onSubmit={handleSubmit(onSubmit)}
					className="flex flex-col w-full justify-center items-center"
				>
					<input
						{...register("email", {
							required: "Email is required",
						})}
						type="email"
						className="input-bordered text-slate-900 input-sm input w-2/6 mt-4 mb-2 active:decoration-none"
						placeholder="Email"
					/>

					<input
						{...register("password", {
							required: "password is required",
							minLength: {
								value: 5,
								message: "Password should be minimum 5 characters",
							},
						})}
						type="password"
						className="input-bordered text-slate-900 input-sm input w-2/6 my-2 active:decoration-none"
						placeholder="Password"
					/>
					<ErrorMessage
						errors={errors}
						name="email"
						render={({ messages }) =>
							messages &&
							Object.entries(messages).map(([type, message]) => (
								<div className="w-2/6">
									<div
										key={type}
										className="alert p-1 m-1 alert-error shadow-lg"
									>
										<div>
											<svg
												xmlns="http://www.w3.org/2000/svg"
												className="stroke-current flex-shrink-0 h-6 w-6"
												fill="none"
												viewBox="0 0 24 24"
											>
												<path
													strokeLinecap="round"
													strokeLinejoin="round"
													strokeWidth="2"
													d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
												/>
											</svg>
											<span className="text-sm">{message}</span>
										</div>
									</div>
								</div>
							))
						}
					/>
					<ErrorMessage
						errors={errors}
						name="password"
						render={({ messages }) =>
							messages &&
							Object.entries(messages).map(([type, message]) => (
								<div className="w-2/6">
									<div
										key={type}
										className="alert p-1 m-1 alert-error shadow-lg"
									>
										<div>
											<svg
												xmlns="http://www.w3.org/2000/svg"
												className="stroke-current flex-shrink-0 h-6 w-6"
												fill="none"
												viewBox="0 0 24 24"
											>
												<path
													strokeLinecap="round"
													strokeLinejoin="round"
													strokeWidth="2"
													d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
												/>
											</svg>
											<span className="text-sm">{message}</span>
										</div>
									</div>
								</div>
							))
						}
					/>
					<button type="submit" className="bg-violet-700 w-2/6 btn">
						Login
					</button>
				</form>
				<div className="w-2/6 my-2 flex items-center justify-center">
					<span className="">or</span>
				</div>
				<button className="bg-slate-200 w-2/6 btn btn-disabled">
					Continue With Google
				</button>
				<p>
					Dont Have an Account? <a href="/register">Register</a>
				</p>
			</div>
		</div>
	);
}

export default Login;

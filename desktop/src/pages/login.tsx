import { useEffect } from "react";
import Image from "next/image";
import axios from "axios";
import { useForm, SubmitHandler } from "react-hook-form";
import { ErrorMessage } from "@hookform/error-message";
// import { window } from "@tauri-apps/api";
import { Procedures } from "@iris/iris_core/bindings";
// import Logo from "../assets/"
import Logo from "../assets/Iconex/Logo-64.png";
import Background from "../assets/auth-bg.png";
import { DeviceInfo, get_device_info, rspc } from "../utils";
import { useRouter } from "next/router";
import AuthLayout from "../layouts/authLayout";
import IRInput from "@components/IRInput";

let test: Procedures | null = null;

type LoginData = {
	email?: string | undefined;
	password?: string | undefined;
};

function Login() {
	const { push } = useRouter();
	const { mutate, data } = rspc.useMutation(["auth.login"]);
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
			username: "",
		});
	};
	console.log("user created: ", data);
	console.log(errors);

	if (typeof data !== "undefined") {
		console.log(data);
		localStorage.setItem("jwt", data);
		push("/home");
	}
	// bg-gradient-to-t from-[#0D1027] to-[#071E42]
	return (
		<AuthLayout>
			<Image src={Logo} height={64} width={64} alt="logo" />
			<p className="text-4xl font-Redrose">Sign In</p>
			<form
				onSubmit={handleSubmit(onSubmit)}
				className="flex flex-col w-3/6 justify-center items-center"
			>
				<IRInput
					label="Email"
					name="email"
					type="text"
					placeholder="eg:elon@musk.com"
					isPassword={false}
					register={register}
					errors={errors.email?.message}
				/>
				<div className="my-2"></div>
				<IRInput
					label="Password"
					name="password"
					type="password"
					placeholder="iHateZuck666"
					isPassword={false}
					register={register}
					errors={errors.email?.message}
				/>
				<div className="my-2"></div>
				<ErrorMessage
					errors={errors}
					name="email"
					render={({ messages }) =>
						messages &&
						Object.entries(messages).map(([type, message]) => (
							<div className="w-2/6">
								<div key={type} className="alert p-1 m-1 alert-error shadow-lg">
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
								<div key={type} className="alert p-1 m-1 alert-error shadow-lg">
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
				<button
					type="submit"
					className="bg-violet-700 w-full min-h-0 btn h-9 text-base leading-[0px] mt-5"
				>
					Login
				</button>
			</form>
			<div className="w-3/6 my-2 flex items-center justify-center">
				<span className="">or</span>
			</div>
			<button className="bg-slate-200 w-3/6 min-h-0 btn btn-disabled h-9 text-base leading-[0px] mb-3">
				Continue With Google
			</button>
			<p>
				Dont Have an Account? <a onClick={() => push("/register")}>Register</a>
			</p>
		</AuthLayout>
	);
}

export default Login;

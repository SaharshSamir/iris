import { useState } from "react";
import {
	FieldError,
	FieldErrorsImpl,
	FieldValues,
	Merge,
	RegisterOptions,
	UseFormRegister,
} from "react-hook-form";

function InfoIcon() {
	return (
		<svg
			fill="none"
			stroke="currentColor"
			stroke-width="1.5"
			viewBox="0 0 24 24"
			xmlns="http://www.w3.org/2000/svg"
			aria-hidden="true"
			className="h-4 w-4 font-bold inline mr-1"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z"
			></path>
		</svg>
	);
}

function Eye({ open }: { open: boolean }) {
	return !open ? (
		<i
			className="far fa-eye icon absolute ml-[-3rem] top-2 p-[10px] text-slate-500"
			id="togglePassword"
		></i>
	) : (
		<i
			className="far fa-eye-slash icon absolute ml-[-3rem] top-2 p-[10px] text-slate-500"
			id="togglePassword"
		></i>
	);
}

interface Props<T extends FieldValues | null> {
	type: string;
	label: string;
	placeholder: string;
	isPassword: boolean;
	name: string;
	// formState: T extends FieldValues ? FormState<T> : null;
	errors:
		| string
		| FieldError
		| Merge<FieldError, FieldErrorsImpl<any>>
		| undefined;
	register?: T extends FieldValues ? UseFormRegister<T> : null;
	validation?: RegisterOptions<FieldValues, string> | undefined;
}

function IRInput<T extends FieldValues | null>(props: Props<T>) {
	const [show, setShow] = useState(props.type !== "password");
	console.log("errors", props.errors);
	const register =
		props.register && props.register(props.name, props.validation);
	return (
		<div className="flex flex-col w-full my-1">
			<label
				htmlFor="eb_input"
				className="font-notoSans font-semibold mb-2 text-sm"
			>
				{props.label}
			</label>
			<div className="relative w-full ">
				<input
					id="eb_input"
					type={show ? "text" : "password"}
					className="border border-[#BEBEBF] rounded-md p-2 px-5 w-full text-sm"
					placeholder={props.placeholder}
					{...register}
				/>
				{props.isPassword && (
					<span className="cursor-pointer" onClick={() => setShow(!show)}>
						<Eye open={show} />
					</span>
				)}
				<p className="text-red-400 text-xs ml-1 mt-1">
					{props.errors ? <InfoIcon /> : null}
					{props.errors?.toString()}
				</p>
			</div>
		</div>
	);
}

export default IRInput;

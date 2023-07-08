import { ReactNode } from "react";
import Image from "next/image";
import Logo from "../assets/Iconex/Logo-32-transparent.png";
import Header from "@components/header";

const BasicLayout = ({ children }: { children: ReactNode }) => {
	return (
		<div
			className=""
			style={{
				backgroundColor: "#0E1118",
				height: "100vh",
				padding: "30px 50px",
			}}
		>
			<div
				style={{
					display: "flex",
					justifyContent: "center",
					alignItems: "center",
				}}
			>
				<Header />
			</div>
			{children}
		</div>
	);
};

export default BasicLayout;

import { ReactNode } from "react";
import Image from "next/image";
import Logo from "../assets/Iconex/Logo-32-transparent.png";

const BasicLayout = ({ children }: { children: ReactNode }) => {
	return (
		<div
			className=""
			style={{
				backgroundColor: "#161E35",
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
				<Image src={Logo} height={32} width={32} alt="Logo" />
				<p style={{ fontFamily: "Rubik Mono One", fontSize: "1.5rem" }}>IRIS</p>
			</div>
			{children}
		</div>
	);
};

export default BasicLayout;

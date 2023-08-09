import { ReactNode } from "react";

function AuthLayout({ children }: { children: ReactNode }) {
	return (
		<div className="overflow-y-auto h-screen p-5 flex flex-col bg-grainy-blobs items-center justify-center">
			{children}
		</div>
	);
}

export default AuthLayout;

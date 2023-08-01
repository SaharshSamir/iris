import Image from "next/image";
import Logo from "@assets/Iconex/Logo-64-transparent.png";
import DefaultUser from "@assets/DefaultUser.png";
import { UserPlusIcon, BellIcon } from "@heroicons/react/24/outline";
import { useRouter } from "next/router";

function SearchInput() {
	return (
		<input
			className="rounded-full ml-10 w-[24rem] h-9 bg-[#2C344A] border-[#5d5d5d]"
			placeholder="search..."
		/>
	);
}

function UserCircle() {
	const { reload } = useRouter();
	return (
		<div className="dropdown dropdown-end rounded-full h-8 w-8 border border-[#5d5d5d] cursor-pointer">
			<label tabIndex={0} className="cursor-pointer">
				<Image src={DefaultUser} height={44} width={44} alt="user profile" />
			</label>
			<ul
				tabIndex={0}
				className="dropdown-content z-[1] menu p-2 shadow bg-[#1F232D] rounded-md w-44"
			>
				<li className="m-1 p-2 rounded-md hover:bg-[#6031FF]">My Profile</li>
				<li className="m-1 p-2 rounded-md hover:bg-[#6031FF]">Settings</li>
				<li
					onClick={() => {
						localStorage.removeItem("jwt");
						reload();
					}}
					className="m-1 p-2 rounded-md hover:bg-[#6031FF]"
				>
					Log Out
				</li>
			</ul>
		</div>
	);
}

<div className="dropdown dropdown-end">
	<label tabIndex={0} className="btn btn-circle btn-ghost btn-xs text-info">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			className="w-4 h-4 stroke-current"
		>
			<path
				strokeLinecap="round"
				strokeLinejoin="round"
				strokeWidth="2"
				d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
			></path>
		</svg>
	</label>
	<div
		tabIndex={0}
		className="card compact dropdown-content z-[1] shadow bg-base-100 rounded-box w-64"
	>
		<div className="card-body">
			<h2 className="card-title">You needed more info?</h2>
			<p>Here is a description!</p>
		</div>
	</div>
</div>;
const Header = () => {
	return (
		<div className="w-full flex justify-between items-center">
			<div>
				<Image src={Logo} height={45} width={45} alt="home" />
			</div>
			<div>
				<SearchInput />
			</div>
			<div className="flex items-center justify-between w-fit">
				<UserPlusIcon className="h-7 w-7 cursor-pointer" />
				<BellIcon className="h-7 w-7 mx-6 cursor-pointer" />
				<UserCircle />
			</div>
		</div>
	);
};

export default Header;

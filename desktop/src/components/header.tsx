import Image from "next/image";
import Logo from "@assets/Iconex/Logo-64-transparent.png";
import DefaultUser from "@assets/DefaultUser.png";
import { UserPlusIcon, BellIcon } from "@heroicons/react/24/outline";

function SearchInput() {
	return (
		<input
			className="rounded-full ml-10 w-[24rem] h-9 bg-[#2C344A] border-[#5d5d5d]"
			placeholder="search..."
		/>
	);
}

function UserCircle() {
	return (
		<div className="rounded-full h-8 w-8 border border-[#5d5d5d] cursor-pointer">
			<Image src={DefaultUser} height={44} width={44} alt="user profile" />
		</div>
	);
}

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

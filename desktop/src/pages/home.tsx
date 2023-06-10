import { rspc } from "../utils";

const Home = () => {
    let {data, isLoading} = rspc.useQuery(["getUser"]);
    if(isLoading){
        return <>"Loading..."</>
    }
	return <>{JSON.stringify(data)}</>;
};

export default Home;

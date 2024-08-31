import { CustomFilter, CustomHome, SearchBar } from "@/components";
import {Button} from '@nextui-org/button'; 
export default function Home() {
  return (
    <main className="overflow-hidden">
      <CustomHome />
      <div
        className="mt-12 padding-x padding-y max-width"
        id="discover"
      >
        <h1 className='text-4xl font-extrabold'>选择您的伙伴</h1>
        <p>强劲动力，带您回家</p>
      </div>
      <div
        className="home__filters">
          <SearchBar />
        <div className="home__filterso_container">

          <CustomFilter />
        </div>
        <Button color="primary">Click me</Button>
      </div>

    </main>
  );
}

import '@picocss/pico';
import moment from 'moment';
import 'moment/locale/it';
import { Chat } from './components/Chat';
import 'flowbite';

moment.locale('it');

function App() {
	return (
		<div className="container mx-auto flex h-screen flex-col justify-between">
			<div className="mb-4">
				<h1 className="text-4xl font-bold">Wasi chat</h1>
				<p>The following client is interacting with a wasi backend!</p>
			</div>

			<section>
				<Chat />
			</section>
		</div>
	);
}

export default App;

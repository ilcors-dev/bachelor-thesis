import '@picocss/pico';
import { Chat } from './components/Chat';

function App() {
	return (
		<div className="container mx-auto flex flex-col justify-between px-4 py-8">
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

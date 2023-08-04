import React, {FC} from 'react';

const Home: FC = () => {
    return (
        <div style={{padding: 24, textAlign: 'center'}} className={"fade-in"}>
            <p>long content</p>
            {
                // indicates very long content
                Array.from({length: 100}, (_, index) => (
                    <React.Fragment key={index}>
                        {index % 20 === 0 && index ? 'more' : '...'}
                        <br/>
                    </React.Fragment>
                ))
            }
        </div>
    );
};

export default Home;